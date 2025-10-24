mod hello_world;
use axum::{routing::{get,post,delete,put},Router,body::Body};
use hello_world::hello_world;
pub fn create_routes()->Router<> {
    Router::new().route("/hello",get(hello_world))
}
