# rust-htmx

## Summary

Sample project to test and learn about HTMX in combination with Rust. The server is using [axum](https://github.com/tokio-rs/axum) 
and the HTML is templated using [askama](https://github.com/djc/askama).

This project let's you view, create and delete TODO's. For simplicity the TODO's are stored in memory and 
therefore this application is **not meant to be deployed in production**!.

## Running the project

Navigate into the project root directory and execute:

To watch for tailwind changes

```sh
npx tailwindcss -i ./style/tailwind.css -o ./static/global.css --watch
```

To start the development server

```sh
cargo run
```
