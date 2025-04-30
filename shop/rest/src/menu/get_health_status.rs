use actix_web::{
    HttpResponse,
    web::{ServiceConfig, get},
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

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;

    use super::*;

    #[actix_web::test]
    async fn created_successfully() {
        let resp = get_health_status().await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
