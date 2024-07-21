use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct Message {
    text: String,
}

#[get("/message")]
pub async fn get_message() -> impl Responder {
    let message = Message {
        text: "Hello from Rust API!".to_string(),
    };
    HttpResponse::Ok().json(message)
}