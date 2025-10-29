use axum::Json;
use serde::{Deserialize, Serialize};
#[derive(Clone,Serialize,Deserialize)]
pub struct Data {
    message : String,
    count :i32,
    username: String,
}
pub async fn get_json()->Json<Data> {
    let data = Data {
        message:String::from("I am a cool data"),
        count:21,
        username:String::from("Hi there its static pages master , Pranjul")
    };

    Json(data)
}