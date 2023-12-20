use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse};
use common::common_rest::main::rest_responses::{
    resource_not_found, rest_business_error, to_invalid_param_bad_request,
};
use domain::main::order::value_objects::shop_order_id::ShopOrderId;
use usecase::main::order::confirm_order::{ConfirmOrder, ConfirmOrderUseCaseError};

use crate::main::{to_error::ToRestError, validated::Validated};

pub async fn execute<T: ConfirmOrder + Send + Debug>(
    shared_state: web::Data<Arc<Mutex<T>>>,
    req: HttpRequest,
) -> HttpResponse {
    let id: i64 = req.match_info().get("id").unwrap().parse().unwrap();

    let error_list = Arc::new(Mutex::new(vec![]));

    match ShopOrderId::validated(id, error_list.clone()) {
        Ok(order_id) => match shared_state.lock().unwrap().execute(order_id) {
            Ok(_) => HttpResponse::new(StatusCode::NO_CONTENT),
            Err(e) => e.to_rest_error(),
        },
        Err(_) => to_invalid_param_bad_request(error_list),
    }
}

impl ToRestError for ConfirmOrderUseCaseError {
    fn to_rest_error(self) -> HttpResponse {
        match self {
            ConfirmOrderUseCaseError::OrderNotFound => resource_not_found(),
            ConfirmOrderUseCaseError::InvalidOrderState => {
                rest_business_error("Invalid state", "invalid_state")
            }
        }
    }
}
