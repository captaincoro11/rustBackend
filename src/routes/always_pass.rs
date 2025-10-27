use axum::http::{Response,StatusCode};
pub async fn always_pass()->Response<String> {
    Response::builder().status(StatusCode::ALREADY_REPORTED).body(String::from("This is a 201")).unwrap()
}