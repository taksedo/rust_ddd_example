use actix_web::{
    error, get,
    http::{header::ContentType, StatusCode},
    App, HttpResponse, HttpServer,
};
use derive_more::{Display, Error};

type Message = String;

pub struct ValidationError {
    message: Message,
}

// #[derive(Debug, Display, Error)]
// pub enum RestHttpError {
//     #[display(fmt = "An internal error occurred. Please try again later.")]
//     InternalError,
// }

// impl error::ResponseError for RestHttpError {
//     fn status_code(&self) -> StatusCode {
//         match *self {
//             RestHttpError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
//         }
//     }
//
//     fn error_response(&self) -> HttpResponse {
//         HttpResponse::build(self.status_code())
//             .insert_header(ContentType::html())
//             .body(self.to_string())
//     }
// }
