use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};
use common::common_rest::main::rest_responses::resource_not_found;
use domain::main::order::value_objects::shop_order_id::ShopOrderId;
use usecase::main::order::get_order_by_id::{GetOrderById, GetOrderByIdUseCaseError};

use crate::main::{
    order::order_model::{OrderModel, ToModel},
    to_error::ToRestError,
};

pub async fn execute<T: GetOrderById + Send + Debug>(
    shared_state: web::Data<Arc<Mutex<T>>>,
    req: HttpRequest,
) -> HttpResponse {
    let id: i64 = req.match_info().get("id").unwrap().parse().unwrap();
    let order_id = ShopOrderId::try_from(id);

    let result = shared_state.lock().unwrap().execute(order_id.unwrap());

    match result {
        Ok(it) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&ToModel::<OrderModel>::to_model(it)).unwrap()),
        Err(e) => e.to_rest_error(),
    }
}

impl ToRestError for GetOrderByIdUseCaseError {
    fn to_rest_error(self) -> HttpResponse {
        match self {
            GetOrderByIdUseCaseError::OrderNotFound => resource_not_found(),
        }
    }
}
