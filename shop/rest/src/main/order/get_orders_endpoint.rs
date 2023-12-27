use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};
use common::common_rest::main::{
    cursor_paged_model::CursorPagedModel,
    rest_responses::{to_invalid_param_bad_request, ValidationError},
};
use domain::main::order::value_objects::shop_order_id::ShopOrderId;
use usecase::main::order::{
    access::shop_order_extractor::ShopOrderExtractor,
    get_orders::{GetOrders, GetOrdersUseCaseError},
    scenarios::get_orders_use_case::GetOrdersUseCase,
};

use super::{
    order_model::{OrderModel, ToModel},
    validated::validate_query_string,
};
use crate::main::{
    endpoint_url::API_V1_ORDER_GET_ALL, to_error::ToRestError, validated::Validated,
};

/// Get orders with pagination
#[utoipa::path(
    get,
    path = API_V1_ORDER_GET_ALL,
    tag = "Order",
    responses(
        (
            status = OK,
            body = Vec<OrderModel>,
            description = "OK" 
        ),
        (
            status = BAD_REQUEST,
            description = "Bad request",
            body = GenericErrorResponse,
            example = json!(
                {
                    "type": "http://0.0.0.0:8080/bad_request",
                    "title": "Bad request",
                    "status": 400,
                    "invalid_params": 
                    [
                        {"message": "Mandatory parameter 'startId' in query is absent"},
                        {"message": "Mandatory parameter 'limit' in query is absent"}
                    ]
                }
            )
        ),
    ),
    params(
        ("limit" = usize, Query, description = "Pagination limit"),
        ("startId" = i64, Query, description = "Pagination start ID")
    )
)]
pub async fn get_orders_endpoint<T: GetOrders + Send + Debug>(
    shared_state: web::Data<Arc<Mutex<T>>>,
    req: HttpRequest,
) -> HttpResponse {
    let error_list = Arc::new(Mutex::new(vec![]));

    match (
        match validate_query_string::<i64>(req.clone(), "startId", error_list.clone()) {
            Ok(id) => ShopOrderId::validated(id, error_list.clone()),
            Err(_) => Err(()),
        },
        validate_query_string::<usize>(req, "limit", error_list.clone()),
    ) {
        (Ok(start_id), Ok(limit)) => {
            match shared_state.lock().unwrap().execute(start_id, limit + 1) {
                Ok(order_details_list) => {
                    let list: Vec<OrderModel> = order_details_list
                        .into_iter()
                        .map(|it| it.to_model())
                        .collect();
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
                Err(e) => e.to_rest_error(),
            }
        }
        (_, _) => to_invalid_param_bad_request(error_list),
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

pub fn get_orders_endpoint_config<ShOExtractor: ShopOrderExtractor + 'static>(
    cfg: &mut web::ServiceConfig,
) {
    cfg.route(
        API_V1_ORDER_GET_ALL,
        web::get().to(get_orders_endpoint::<GetOrdersUseCase<ShOExtractor>>),
    );
}
