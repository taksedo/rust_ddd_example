use std::{
    collections::HashMap,
    fmt::Debug,
    num::ParseIntError,
    sync::{Arc, Mutex},
};

use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};
use common::common_rest::main::rest_responses::{to_invalid_param_bad_request, ValidationError};
use domain::main::order::value_objects::shop_order_id::ShopOrderId;
use serde_derive::{Deserialize, Serialize};
use usecase::main::order::get_orders::{GetOrders, GetOrdersUseCaseError};

use super::order_model::{OrderModel, ToModel};
use crate::main::to_error::ToRestError;

pub async fn execute<T: GetOrders + Send + Debug>(
    shared_state: web::Data<Arc<Mutex<T>>>,
    req: HttpRequest,
) -> HttpResponse {
    let error_list = Arc::new(Mutex::new(vec![]));

    let mut start_id: Result<i64, ParseIntError> = Ok(0);
    let mut limit: Result<usize, ParseIntError> = Ok(0);

    let params = web::Query::<HashMap<String, String>>::from_query(req.query_string()).unwrap();

    if let Some(val) = params.get("startId") {
        start_id = match val.parse::<i64>() {
            Err(err) => {
                error_list
                    .lock()
                    .unwrap()
                    .push(ValidationError::new(&err.to_string()));
                Err(err)
            }
            Ok(start_id) => Ok(start_id),
        };
    } else {
        error_list
            .lock()
            .unwrap()
            .push(ValidationError::new("startId is absent"));
    }

    if let Some(val) = params.get("limit") {
        limit = match val.parse::<usize>() {
            Err(err) => {
                error_list
                    .lock()
                    .unwrap()
                    .push(ValidationError::new(&err.to_string()));
                Err(err)
            }
            Ok(limit) => Ok(limit),
        };
    } else {
        error_list
            .lock()
            .unwrap()
            .push(ValidationError::new("limit is absent"));
    }

    if error_list.lock().unwrap().is_empty() {
        let start_id = start_id.unwrap();
        let limit = limit.unwrap();

        match shared_state
            .lock()
            .unwrap()
            .execute(ShopOrderId::try_from(start_id).unwrap(), limit + 1_usize)
        {
            Ok(order_details) => {
                let list: Vec<OrderModel> =
                    order_details.into_iter().map(|it| it.to_model()).collect();
                let model = if list.len() > limit {
                    let next_id = list[limit].id;
                    CursorPagedModel::new(list[..limit].to_vec(), Some(next_id))
                } else {
                    CursorPagedModel::new(list, Option::<i64>::None)
                };
                HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .body(serde_json::to_string(&model).unwrap())
            }
            Err(err) => err.to_rest_error(),
        }
    } else {
        to_invalid_param_bad_request(error_list)
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CursorPagedModel<T, ID> {
    pub list: Vec<T>,
    pub next: Option<ID>,
    pub count: usize,
}

impl<T, ID> CursorPagedModel<T, ID> {
    pub fn new(list: Vec<T>, next: Option<ID>) -> Self {
        let count = list.len();
        Self { list, next, count }
    }
}
