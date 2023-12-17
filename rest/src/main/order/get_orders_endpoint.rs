use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};
use common::common_rest::main::rest_responses::{to_invalid_param_bad_request, ValidationError};
use domain::main::order::value_objects::shop_order_id::ShopOrderId;
use usecase::main::order::get_orders::{GetOrders, GetOrdersUseCaseError};

use super::order_model::{OrderModel, ToModel};
use crate::main::to_error::ToRestError;

pub async fn execute<T: GetOrders + Send + Debug>(
    shared_state: web::Data<Arc<Mutex<T>>>,
    req: HttpRequest,
) -> HttpResponse {
    let start_id: i64 = req.match_info().get("start_id").unwrap().parse().unwrap();
    let limit = req.match_info().get("limit").unwrap().parse().unwrap();

    let result = shared_state
        .lock()
        .unwrap()
        .execute(ShopOrderId::try_from(start_id).unwrap(), limit)
        .map_err(|e| e.to_rest_error());
    if let Err(error) = result {
        error
    } else {
        let order_model_list: Vec<OrderModel> = result
            .unwrap()
            .into_iter()
            .map(|it| it.to_model())
            .collect();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&order_model_list).unwrap())
    }
}

impl ToRestError for GetOrdersUseCaseError {
    fn to_rest_error(self) -> HttpResponse {
        match self {
            GetOrdersUseCaseError::LimitExceed(max_size) => {
                let error_list = Arc::new(Mutex::new(vec![]));
                error_list
                    .lock()
                    .unwrap()
                    .push(ValidationError::new(&format!(
                        "Max limit is {}",
                        max_size - 1
                    )));
                to_invalid_param_bad_request(error_list)
            }
        }
    }
}
