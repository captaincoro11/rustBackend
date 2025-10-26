use axum::{
    body::Body,
    extract::Request,
    http::{HeaderValue, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn set_middleware_custom_header(
    request: Request<Body>, // <- must be Body
    next: Next,
) -> Result<Response, StatusCode> {
    // Run the next handler
    let mut response = next.run(request).await;

    // Add a custom header
    response.headers_mut().insert(
        "x-custom-header",
        HeaderValue::from_static("my-value"),
    );

    Ok(response)
}
