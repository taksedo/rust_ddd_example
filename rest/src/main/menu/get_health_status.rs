use actix_web::{HttpResponse, Responder};

pub async fn execute() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("Healthy!")
}
