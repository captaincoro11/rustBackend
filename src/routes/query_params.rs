use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct QueryMain{
    message: String,
    id:i32
}
pub async fn query_params(Query(query):Query<QueryMain>)->Json<QueryMain> {
    Json(query)
}