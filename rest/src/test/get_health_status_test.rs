use crate::main::menu::get_health_status;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;

#[actix_web::test]
async fn created_successfully() {
    let resp = get_health_status::execute().await;
    assert_eq!(resp.status(), StatusCode::OK);
}
