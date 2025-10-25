use serde::{Serialize,Deserialize};
use axum:: Json;
#[derive(Serialize,Deserialize,Debug)]
pub struct MirrorJson{
    message : String,
} 

pub async fn mirror_body_json(Json(body):Json<MirrorJson>)->Json<MirrorJson> {
    Json(body)
}