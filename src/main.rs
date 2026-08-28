use askama::Template;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tower_http::services::ServeDir;

#[derive(Template)]
#[template(path = "editor.html")]
struct EditorTemplate;

async fn editor() -> impl IntoResponse {
    Html(EditorTemplate.render().expect("render editor"))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(editor))
        .nest_service("/static", ServeDir::new("static"));

    let address =
        std::env::var("BAPTISM_PROGRAM_ADDRESS").unwrap_or_else(|_| "127.0.0.1:3000".to_owned());
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .expect("bind server");

    println!("Baptism Program Builder running at http://{address}");
    axum::serve(listener, app).await.expect("server failed");
}
