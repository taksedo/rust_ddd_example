use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse};
use common::common_rest::rest_responses::{
    get_json_from_http_response, resource_not_found, rest_business_error,
    to_invalid_param_bad_request,
};
use domain::order::value_objects::shop_order_id::ShopOrderId;
use usecase::order::{
    access::{shop_order_extractor::ShopOrderExtractor, shop_order_persister::ShopOrderPersister},
    cancel_order::{CancelOrder, CancelOrderUseCaseError},
    scenarios::cancel_order_use_case::CancelOrderUseCase,
};

use crate::{endpoint_url::API_V1_ORDER_CANCEL_BY_ID, to_error::ToRestError, validated::Validated};

/// Cancel an order by id
#[utoipa::path(
    put,
    path = API_V1_ORDER_CANCEL_BY_ID,
    tag = "Order",
    responses(
        (
            status = NO_CONTENT,
            description = "Sucessfully cancelled" 
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
pub async fn cancel_order_endpoint<T: CancelOrder + Send + Debug>(
    shared_state: web::Data<Arc<Mutex<T>>>,
    req: HttpRequest,
) -> HttpResponse {
    let id: i64 = req.match_info().get("id").unwrap().parse().unwrap();

    let error_list = Arc::new(Mutex::new(vec![]));

    match ShopOrderId::validated(id, error_list.clone()) {
        Ok(order_id) => match shared_state.lock().unwrap().execute(&order_id) {
            Ok(_) => HttpResponse::new(StatusCode::NO_CONTENT),
            Err(e) => e.to_rest_error(),
        },
        Err(_) => to_invalid_param_bad_request(error_list),
    }
}

impl ToRestError for CancelOrderUseCaseError {
    fn to_rest_error(self) -> HttpResponse {
        match self {
            CancelOrderUseCaseError::OrderNotFound => resource_not_found(),
            CancelOrderUseCaseError::InvalidOrderState => {
                rest_business_error("Invalid state", "invalid_state")
            }
        }
    }
}

pub fn cancel_order_endpoint_config<ShOExtractor, ShOPersister>(cfg: &mut web::ServiceConfig)
where
    ShOExtractor: ShopOrderExtractor + 'static,
    ShOPersister: ShopOrderPersister + 'static,
{
    cfg.route(
        API_V1_ORDER_CANCEL_BY_ID,
        web::put().to(cancel_order_endpoint::<CancelOrderUseCase<ShOExtractor, ShOPersister>>),
    );
}

#[cfg(test)]
mod tests {
    use actix_web::{body::MessageBody, test::TestRequest, web::Data};
    use common::common_rest::rest_responses::{
        error_type_url, not_found_type_url, GenericErrorResponse,
    };
    use dotenvy::dotenv;

    use super::*;
    use crate::{domain_test_fixtures::rnd_order_id, test_fixtures::MockCancelOrder};

    #[actix_web::test]
    async fn order_not_found() {
        dotenv().ok();
        let order_id = rnd_order_id();
        let mock_cancel_order = Arc::new(Mutex::new(MockCancelOrder::default()));
        mock_cancel_order.lock().unwrap().response = Err(CancelOrderUseCaseError::OrderNotFound);

        let mock_shared_state = Data::new(mock_cancel_order.clone());

        let req = TestRequest::default()
            .param("id", order_id.to_i64().to_string())
            .to_http_request();

        let resp = cancel_order_endpoint(mock_shared_state, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        let body = resp.into_body().try_into_bytes().unwrap();
        let body_text = std::str::from_utf8(&body).unwrap();

        let response_dto: GenericErrorResponse = serde_json::from_str(body_text).unwrap();

        assert_eq!(&response_dto.response_type, &not_found_type_url());
        assert_eq!(
            &response_dto.response_status,
            &StatusCode::NOT_FOUND.as_u16()
        );
        assert_eq!(&response_dto.response_title, "Resource not found");
    }

    #[actix_web::test]
    async fn invalid_order_state() {
        dotenv().ok();
        let order_id = rnd_order_id();
        let mock_cancel_order = Arc::new(Mutex::new(MockCancelOrder::default()));
        mock_cancel_order.lock().unwrap().response =
            Err(CancelOrderUseCaseError::InvalidOrderState);

        let mock_shared_state = Data::new(mock_cancel_order.clone());

        let req = TestRequest::default()
            .param("id", order_id.to_i64().to_string())
            .to_http_request();

        let resp = cancel_order_endpoint(mock_shared_state, req).await;

        assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);

        let body = resp.into_body().try_into_bytes().unwrap();
        let body_text = std::str::from_utf8(&body).unwrap();

        let response_dto: GenericErrorResponse = serde_json::from_str(body_text).unwrap();

        assert_eq!(
            &response_dto.response_type,
            &error_type_url("invalid_state")
        );
        assert_eq!(
            &response_dto.response_status,
            &StatusCode::UNPROCESSABLE_ENTITY.as_u16()
        );
        assert_eq!(&response_dto.response_title, "Invalid state");
    }

    #[actix_web::test]
    async fn successfully_cancelled() {
        dotenv().ok();
        let order_id = rnd_order_id();
        let mock_cancel_order = Arc::new(Mutex::new(MockCancelOrder::default()));
        mock_cancel_order.lock().unwrap().response = Ok(());

        let mock_shared_state = Data::new(mock_cancel_order.clone());

        let req = TestRequest::default()
            .param("id", order_id.to_i64().to_string())
            .to_http_request();

        let resp = cancel_order_endpoint(mock_shared_state, req).await;

        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        let body = resp.into_body().try_into_bytes().unwrap();

        assert!(body.is_empty());
    }
}
