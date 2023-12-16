use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use actix_web::{http::header::ContentType, web, HttpResponse};
use common::common_rest::main::rest_responses::{to_invalid_param_bad_request, ValidationError};
use domain::main::order::value_objects::shop_order_id::ShopOrderId;
use usecase::main::order::get_orders::{GetOrders, GetOrdersUseCaseError};

use super::order_model::{OrderModel, ToModel};
use crate::main::to_error::ToRestError;

pub async fn execute<T: GetOrders + Send + Debug>(
    shared_state: web::Data<Arc<Mutex<T>>>,
    start_id: ShopOrderId,
    limit: usize,
) -> HttpResponse {
    let order_model_list: Vec<OrderModel> = shared_state
        .lock()
        .unwrap()
        .execute(start_id, limit)
        .map_err(|e| e.to_rest_error())
        .unwrap()
        .into_iter()
        .map(|it| it.to_model())
        .collect();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&order_model_list).unwrap())
}

impl ToRestError for GetOrdersUseCaseError {
    fn to_rest_error(self) -> HttpResponse {
        match self {
            GetOrdersUseCaseError::LimitExceed(limit) => {
                let error_list = Arc::new(Mutex::new(vec![]));
                error_list
                    .lock()
                    .unwrap()
                    .push(ValidationError::new(&format!("Max limit is {limit}")));
                to_invalid_param_bad_request(error_list)
            }
        }
    }
}
