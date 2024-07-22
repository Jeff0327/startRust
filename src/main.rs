use actix_web::{App, HttpServer};
use actix_cors::Cors;
use dotenv::dotenv;
use std::env;
pub mod api {
    pub mod message;
    pub mod search;
}

use api::message::get_message;
use api::search::search;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let server_url=env::var("SERVER_URL").expect("Server URL cannot be empty");
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive()) // 개발 환경에서만 사용. 실제 환경에서는 더 엄격한 CORS 설정 필요
            .service(get_message)
            .service(search)
    })
    .bind(server_url)?
    .run()
    .await
}