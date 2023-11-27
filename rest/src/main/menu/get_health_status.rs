use actix_web::HttpResponse;

pub async fn execute() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("Healthy!")
}
