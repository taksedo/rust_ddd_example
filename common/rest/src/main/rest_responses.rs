use actix_web::http::StatusCode;
use actix_web::http::Uri;
use actix_web::HttpResponse;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::OnceLock;

lazy_static! {
    pub static ref BASE_URL: String = {
        let lock: OnceLock<String> = OnceLock::new();
        lock.get_or_init(|| env::var("HOST_URL").unwrap())
            .to_string()
    };
}

pub fn resource_not_found() -> HttpResponse {
    let error_response = GenericErrorResponse {
        response_type: (BASE_URL.clone() + "/resource_not_found")
            .parse::<Uri>()
            .unwrap()
            .to_string(),
        response_title: "Resource not found",
        response_status: (StatusCode::NOT_FOUND).as_u16(),
    };
    HttpResponse::NotFound().json(error_response)
}

pub fn rest_business_error(title: &str, code: &str) -> HttpResponse {
    let error_response = GenericErrorResponse {
        response_type: (BASE_URL.clone() + "/" + code)
            .parse::<Uri>()
            .unwrap()
            .to_string(),
        response_title: title,
        response_status: (StatusCode::UNPROCESSABLE_ENTITY).as_u16(),
    };
    HttpResponse::UnprocessableEntity().json(error_response)
}

pub fn created(location: Uri) -> HttpResponse {
    HttpResponse::Created()
        .append_header(("Location", location.to_string()))
        .finish()
}

pub fn no_content() -> HttpResponse {
    HttpResponse::new(StatusCode::NO_CONTENT)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenericErrorResponse<'a> {
    #[serde(rename(serialize = "type"))]
    pub response_type: String,
    #[serde(rename(serialize = "title"))]
    pub response_title: &'a str,
    #[serde(rename(serialize = "status"))]
    pub response_status: u16,
}

pub struct ValidationError {
    message: String,
}

impl ValidationError {
    fn to_invalid_param_bad_request(self) -> HttpResponse {
        let error_response = GenericErrorResponse {
            response_title: "Bad request",
            response_status: (StatusCode::BAD_REQUEST).as_u16(),
            response_type: (BASE_URL.clone() + "/bad_request")
                .parse::<Uri>()
                .unwrap()
                .to_string(),
        };
        HttpResponse::BadRequest().json(error_response)
    }
}
