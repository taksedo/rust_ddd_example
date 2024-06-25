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
use usecase::main::order::{
    access::{shop_order_extractor::ShopOrderExtractor, shop_order_persister::ShopOrderPersister},
    confirm_order::{ConfirmOrder, ConfirmOrderUseCaseError},
    scenarios::confirm_order_use_case::ConfirmOrderUseCase,
};

use crate::{
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
        Ok(order_id) => match shared_state.lock().unwrap().execute(&order_id) {
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

#[cfg(test)]
mod tests {
    use actix_web::{body::MessageBody, test::TestRequest, web::Data};
    use common::common_rest::rest_responses::{
        error_type_url, not_found_type_url, GenericErrorResponse,
    };
    use domain::test_fixtures::rnd_order_id;
    use dotenvy::dotenv;

    use super::*;
    use crate::test_fixtures::MockConfirmOrder;

    #[actix_web::test]
    async fn order_not_found() {
        dotenv().ok();
        let order_id = rnd_order_id();
        let mock_confirm_order = Arc::new(Mutex::new(MockConfirmOrder::default()));
        mock_confirm_order.lock().unwrap().response = Err(ConfirmOrderUseCaseError::OrderNotFound);

        let mock_shared_state = Data::new(mock_confirm_order.clone());

        let req = TestRequest::default()
            .param("id", order_id.to_i64().to_string())
            .to_http_request();

        let resp: actix_web::HttpResponse = confirm_order_endpoint(mock_shared_state, req).await;

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

        mock_confirm_order.lock().unwrap().verify_invoked(&order_id);
    }

    #[actix_web::test]
    async fn invalid_order_state() {
        dotenv().ok();
        let order_id = rnd_order_id();
        let mock_confirm_order = Arc::new(Mutex::new(MockConfirmOrder::default()));
        mock_confirm_order.lock().unwrap().response =
            Err(ConfirmOrderUseCaseError::InvalidOrderState);

        let mock_shared_state = Data::new(mock_confirm_order.clone());

        let req = TestRequest::default()
            .param("id", order_id.to_i64().to_string())
            .to_http_request();

        let resp = confirm_order_endpoint(mock_shared_state, req).await;

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

        mock_confirm_order.lock().unwrap().verify_invoked(&order_id);
    }

    #[actix_web::test]
    async fn successfully_cancelled() {
        dotenv().ok();
        let order_id = rnd_order_id();
        let mock_confirm_order = Arc::new(Mutex::new(MockConfirmOrder::default()));
        mock_confirm_order.lock().unwrap().response = Ok(());

        let mock_shared_state = Data::new(mock_confirm_order.clone());

        let req = TestRequest::default()
            .param("id", order_id.to_i64().to_string())
            .to_http_request();

        let resp = confirm_order_endpoint(mock_shared_state, req).await;

        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        let body = resp.into_body().try_into_bytes().unwrap();

        assert!(body.is_empty());
        mock_confirm_order.lock().unwrap().verify_invoked(&order_id);
    }
}
