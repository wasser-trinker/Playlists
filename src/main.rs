use axum::routing::get;

#[tokio::main]
async fn main() {
    let router = axum::Router::new()
        .nest_service("/assets", tower_http::services::ServeDir::new("./assets"))
        .route("/", get(async || "Hello :3"));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:1987").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
