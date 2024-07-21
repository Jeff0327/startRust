use actix_web::{web, post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchRequest {
    pub id: i32,
}

#[derive(Debug,Deserialize, Serialize, Clone)]
pub struct SearchResponse {
    pub id: i32,
    pub name: String,
}

#[post("/search")]
pub async fn search(req: web::Json<SearchRequest>) -> impl Responder {
    // 여기서는 간단한 예시로 구현합니다.
    // 실제로는 데이터베이스 조회 등의 로직이 들어갈 수 있습니다.
    let response = SearchResponse {
        id: req.id,
        name: format!("User {}", req.id),
    };
    HttpResponse::Ok().json(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_search() {
        let app = test::init_service(App::new().service(search)).await;

        // 존재하는 사용자 검색
        let req = test::TestRequest::post()
            .uri("/search")
            .set_json(SearchRequest { id: 1 })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let body: SearchResponse = test::read_body_json(resp).await;
        assert_eq!(body.id, 1);
        assert_eq!(body.name, "User 1");

        // 존재하지 않는 사용자 검색
        let req = test::TestRequest::post()
            .uri("/search")
            .set_json(SearchRequest { id: 100 })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}