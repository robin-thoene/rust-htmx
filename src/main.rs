use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread, time,
};

use askama_axum::Template;
use axum::{
    extract::Path,
    routing::{delete, get, post},
    Extension, Form, Router,
};
use serde::Deserialize;
use tower_http::services::ServeDir;
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
    todo_list_items: Vec<TodoListItemTemplate>,
}

#[derive(Template)]
#[template(path = "todo_list_item.html")]
struct TodoListItemTemplate {
    todo: TodoTemplate,
}

struct InMemDb {
    todos: HashMap<String, String>,
}

async fn index(db: Extension<Arc<Mutex<InMemDb>>>) -> IndexTemplate {
    let todo_list_items =
        db.0.lock()
            .unwrap()
            .todos
            .iter()
            .map(|(key, val)| TodoListItemTemplate {
                todo: TodoTemplate {
                    id: key.to_string(),
                    title: val.to_string(),
                },
            })
            .collect();
    IndexTemplate { todo_list_items }
}

async fn delete_todo(db: Extension<Arc<Mutex<InMemDb>>>, Path(id): Path<String>) {
    db.0.lock().unwrap().todos.remove(&id);
}

#[derive(Deserialize)]
struct CreateForm {
    title: String,
}

async fn create_todo(
    db: Extension<Arc<Mutex<InMemDb>>>,
    Form(create_form): Form<CreateForm>,
) -> TodoListItemTemplate {
    let ten_millis = time::Duration::from_millis(3000);
    thread::sleep(ten_millis);
    let id = Uuid::new_v4().to_string();
    db.0.lock()
        .unwrap()
        .todos
        .insert(id.clone(), create_form.title.clone());
    TodoListItemTemplate {
        todo: TodoTemplate {
            id,
            title: create_form.title,
        },
    }
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
        .route("/api/todo", post(create_todo))
        .layer(Extension(Arc::clone(&db)))
        .nest_service("/static", ServeDir::new("static/"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:42069")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
