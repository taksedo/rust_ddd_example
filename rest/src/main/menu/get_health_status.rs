use actix_web::{
    web::{get, ServiceConfig},
    HttpResponse,
};

pub async fn get_health_status() -> HttpResponse {
    HttpResponse::Ok().json("Healthy!")
}

pub fn get_health_status_config(cfg: &mut ServiceConfig) {
    cfg.route("/health", get().to(get_health_status));
}
