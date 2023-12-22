use actix_web::{web, HttpResponse};

pub async fn get_health_status() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("Healthy!")
}

pub fn get_health_status_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(get_health_status));
}
