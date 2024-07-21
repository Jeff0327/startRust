use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    text: String,
}

#[get("/message")]
async fn get_message() -> impl Responder {
    let message = Message {
        text: "Hello from Rust API!".to_string(),
    };
    HttpResponse::Ok().json(message)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive()) // 개발 환경에서만 사용. 실제 환경에서는 더 엄격한 CORS 설정 필요
            .service(get_message)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}