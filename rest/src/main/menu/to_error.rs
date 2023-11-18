use actix_web::HttpResponse;

pub trait ToRestError {
    fn to_rest_error(self) -> HttpResponse;
}
