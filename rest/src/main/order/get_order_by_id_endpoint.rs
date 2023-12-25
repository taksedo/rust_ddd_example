use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};
use common::common_rest::main::rest_responses::{resource_not_found, to_invalid_param_bad_request};
use domain::main::order::value_objects::shop_order_id::ShopOrderId;
use usecase::main::order::{
    access::shop_order_extractor::ShopOrderExtractor,
    get_order_by_id::{GetOrderById, GetOrderByIdUseCaseError},
    scenarios::get_order_by_id_use_case::GetOrderByIdUseCase,
};

use crate::main::{
    endpoint_url::API_V1_ORDER_GET_BY_ID,
    order::order_model::{OrderModel, ToModel},
    to_error::ToRestError,
    validated::Validated,
};

pub async fn get_order_by_id_endpoint<T: GetOrderById + Send + Debug>(
    shared_state: web::Data<Arc<Mutex<T>>>,
    req: HttpRequest,
) -> HttpResponse {
    let id: i64 = req.match_info().get("id").unwrap().parse().unwrap();

    let error_list = Arc::new(Mutex::new(vec![]));

    match ShopOrderId::validated(id, error_list.clone()) {
        Ok(order_id) => match shared_state.lock().unwrap().execute(order_id) {
            Ok(it) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&ToModel::<OrderModel>::to_model(it)).unwrap()),
            Err(e) => e.to_rest_error(),
        },
        Err(_) => to_invalid_param_bad_request(error_list),
    }
}

impl ToRestError for GetOrderByIdUseCaseError {
    fn to_rest_error(self) -> HttpResponse {
        match self {
            GetOrderByIdUseCaseError::OrderNotFound => resource_not_found(),
        }
    }
}

pub fn get_order_by_id_endpoint_config<ShOExtractor: ShopOrderExtractor + 'static>(
    cfg: &mut web::ServiceConfig,
) {
    cfg.route(
        API_V1_ORDER_GET_BY_ID,
        web::get().to(get_order_by_id_endpoint::<GetOrderByIdUseCase<ShOExtractor>>),
    );
}
