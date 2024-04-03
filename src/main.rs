use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use askama_axum::Template;
use axum::{
    extract::Path,
    routing::{delete, get},
    Extension, Router,
};
use uuid::Uuid;

#[derive(Template)]
#[template(path = "todo.html")]
struct TodoTemplate {
    id: String,
    title: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    todos: Vec<TodoTemplate>,
}

struct InMemDb {
    todos: HashMap<String, String>,
}

async fn index(db: Extension<Arc<Mutex<InMemDb>>>) -> IndexTemplate {
    let todos =
        db.0.lock()
            .unwrap()
            .todos
            .iter()
            .map(|(key, val)| TodoTemplate {
                id: key.to_string(),
                title: val.to_string(),
            })
            .collect();
    IndexTemplate { todos }
}

async fn delete_todo(db: Extension<Arc<Mutex<InMemDb>>>, Path(id): Path<String>) {
    db.0.lock().unwrap().todos.remove(&id);
}

#[tokio::main]
async fn main() {
    let mut inital_state = HashMap::<String, String>::new();
    inital_state.insert(
        Uuid::new_v4().to_string(),
        "follow robin-thoene on github".to_string(),
    );
    inital_state.insert(
        Uuid::new_v4().to_string(),
        "star this repository".to_string(),
    );
    let db = Arc::new(Mutex::new(InMemDb {
        todos: inital_state,
    }));
    let app = Router::new()
        .route("/", get(index))
        .route("/api/todo/:id", delete(delete_todo))
        .layer(Extension(Arc::clone(&db)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
