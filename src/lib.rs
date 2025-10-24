mod routes;
use axum::{routing::get,Router};

use crate::routes::create_routes;
pub async fn run() {
    let app = create_routes();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener,app).await.unwrap();
}

