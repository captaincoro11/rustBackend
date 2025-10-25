mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod path_variables;
mod query_params;

use axum::{routing::{get,post,delete,put},Router,body::Body};
use hello_world::hello_world;
use mirror_body_string::mirror_body_string;
use mirror_body_json::mirror_body_json;
use path_variables::path_variables;
use query_params::query_params;

pub fn create_routes()->Router<> {
    Router::new().route("/hello",get(hello_world))
    .route("/mirror_body_string", post(mirror_body_string))
    .route("/mirror_body_json",post(mirror_body_json))
    .route("/path_variables/{id}",get(path_variables))
    .route("/query_params",get(query_params))
}
