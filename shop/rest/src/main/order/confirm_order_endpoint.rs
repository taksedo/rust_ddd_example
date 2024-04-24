use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse};
use common::common_rest::rest_responses::{
    get_json_from_http_response, resource_not_found, rest_business_error,
    to_invalid_param_bad_request,
};
use domain::main::order::value_objects::shop_order_id::ShopOrderId;
use usecase::main::order::{
    access::{shop_order_extractor::ShopOrderExtractor, shop_order_persister::ShopOrderPersister},
    confirm_order::{ConfirmOrder, ConfirmOrderUseCaseError},
    scenarios::confirm_order_use_case::ConfirmOrderUseCase,
};

use crate::main::{
    endpoint_url::API_V1_ORDER_CONFIRM_BY_ID, to_error::ToRestError, validated::Validated,
};

/// Confirm an order by id
#[utoipa::path(
    put,
    path = API_V1_ORDER_CONFIRM_BY_ID,
    tag = "Order",
    responses(
        (
            status = NO_CONTENT,
            description = "Successfully confirmed" 
        ),
        (
            status = BAD_REQUEST,
            description = "Bad request",
            body = GenericErrorResponse,
            example = json!(
                {
                    "type":"http://0.0.0.0:8080/bad_request",
                    "title":"Bad request",
                    "status":400,
                    "invalid_params":
                    [
                        {"message": "Wrong Shop Order Id"}
                    ]
                }
            )
        ),
        (
            status = NOT_FOUND,
            description = "Order not found",
            body = GenericErrorResponse,
            example = json!(&(get_json_from_http_response(resource_not_found())))
        ),
    ),
    params(
        ("id" = i64, description = "id"),
    )
)]
pub async fn confirm_order_endpoint<T: ConfirmOrder + Send + Debug>(
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

pub fn confirm_order_endpoint_config<ShOExtractor, ShOPersister>(cfg: &mut web::ServiceConfig)
where
    ShOExtractor: ShopOrderExtractor + 'static,
    ShOPersister: ShopOrderPersister + 'static,
{
    cfg.route(
        API_V1_ORDER_CONFIRM_BY_ID,
        web::put().to(confirm_order_endpoint::<ConfirmOrderUseCase<ShOExtractor, ShOPersister>>),
    );
}
