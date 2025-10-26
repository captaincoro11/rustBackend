mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod path_variables;
mod query_params;
mod mirror_user_agent;
mod mirror_custom_header;
mod middleware_message;
mod middleware_custom_header;
mod set_middleware_custom_header;

use axum::{http::Method, routing::{get,post}, Extension, Router,middleware};
use hello_world::hello_world;
use mirror_body_string::mirror_body_string;
use mirror_body_json::mirror_body_json;
use path_variables::path_variables;
use query_params::query_params;
use mirror_user_agent::mirror_user_agent;
use mirror_custom_header::mirror_custom_header;
use tower_http::cors::{Any, CorsLayer};
use middleware_message::middleware_message;
use middleware_custom_header::middleware_custom_header;
use set_middleware_custom_header::set_middleware_custom_header;

#[derive(Clone)]
pub struct SharedData {
    pub message : String
}

pub fn create_routes()->Router<> {
    let cors = CorsLayer::new().allow_methods([Method::GET,Method::POST]).allow_origin(Any);
    let sharedData = SharedData {
        message: String::from("Hello From Shared Data"),
    };
    Router::new().route("/hello",get(hello_world))
    .route_layer(middleware::from_fn(set_middleware_custom_header))
    .route("/mirror_body_string", post(mirror_body_string))
    .route("/mirror_body_json",post(mirror_body_json))
    .route("/path_variables/{id}",get(path_variables))
    .route("/query_params",get(query_params))
    .route("/mirror_user_agent", post(mirror_user_agent))
    .route("/mirror_custom_header",get(mirror_custom_header))
    .route("/middleware_message",get(middleware_message))
    .route("/middleware_custom_header",get(middleware_custom_header))
    .layer(cors)
    .layer(Extension(sharedData))
}
