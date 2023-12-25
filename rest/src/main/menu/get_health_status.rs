use actix_web::{
    web::{get, ServiceConfig},
    HttpResponse,
};

/// Check health status
#[utoipa::path(get, path = "/health",tag = "Health",
    responses(
    (status = 200, description = "Application is healthy", example = "Healthy!"),
))]
pub async fn get_health_status() -> HttpResponse {
    HttpResponse::Ok().json("Healthy!")
}

pub fn get_health_status_config(cfg: &mut ServiceConfig) {
    cfg.route("/health", get().to(get_health_status));
}
