use axum::Extension;

use crate::routes::SharedData;

pub async fn middleware_message(Extension(sharedData):Extension<SharedData>)->String {
    sharedData.message
}