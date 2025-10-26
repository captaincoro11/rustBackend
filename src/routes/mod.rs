mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod path_variables;
mod query_params;
mod mirror_user_agent;
mod mirror_custom_header;

use axum::{http::Method, routing::{get,post}, Router};
use hello_world::hello_world;
use mirror_body_string::mirror_body_string;
use mirror_body_json::mirror_body_json;
use path_variables::path_variables;
use query_params::query_params;
use mirror_user_agent::mirror_user_agent;
use mirror_custom_header::mirror_custom_header;
use tower_http::cors::{Any, CorsLayer};

pub fn create_routes()->Router<> {
    let cors = CorsLayer::new().allow_methods([Method::GET,Method::POST]).allow_origin(Any);
    Router::new().route("/hello",get(hello_world))
    .route("/mirror_body_string", post(mirror_body_string))
    .route("/mirror_body_json",post(mirror_body_json))
    .route("/path_variables/{id}",get(path_variables))
    .route("/query_params",get(query_params))
    .route("/mirror_user_agent", post(mirror_user_agent))
    .route("/mirror_custom_header",get(mirror_custom_header))
    .layer(cors)
}
