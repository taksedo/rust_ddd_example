use std::fmt::Debug;

use actix_web::{HttpRequest, HttpResponse, http::StatusCode, web};
use common::{
    common_rest::{
        GenericErrorResponse, get_json_from_http_response, resource_not_found, rest_business_error,
        to_invalid_param_bad_request,
    },
    types::base::{AM, AMTrait, RCell, RcRefCellTrait},
};
use domain::order::value_objects::shop_order_id::ShopOrderId;
use usecase::order::{ConfirmOrder, ConfirmOrderUseCaseError};

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
pub async fn confirm_order_endpoint<T>(
    shared_state: web::Data<AM<T>>,
    req: HttpRequest,
) -> HttpResponse
where
    T: ConfirmOrder + Send + Debug,
{
    let id: i64 = req.match_info().get("id").unwrap().parse().unwrap();

    let error_list = RCell::new_rc(vec![]);

    match ShopOrderId::validated(id, error_list.clone()) {
        Ok(order_id) => match shared_state.lock_un().execute(&order_id) {
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

pub fn confirm_order_endpoint_config<T>(cfg: &mut web::ServiceConfig)
where
    T: ConfirmOrder + 'static,
{
    cfg.route(
        API_V1_ORDER_CONFIRM_BY_ID,
        web::put().to(confirm_order_endpoint::<T>),
    );
}

#[cfg(test)]
mod tests {
    use actix_web::{body::MessageBody, test::TestRequest, web::Data};
    use common::common_rest::{GenericErrorResponse, error_type_url, not_found_type_url};
    use domain::test_fixtures::*;
    use dotenvy::dotenv;

    use super::*;
    use crate::test_fixtures::MockConfirmOrder;

    #[actix_web::test]
    async fn order_not_found() {
        dotenv().ok();
        let order_id = rnd_order_id();
        let mock_confirm_order = AM::new_am(MockConfirmOrder::default());
        mock_confirm_order.lock_un().response = Err(ConfirmOrderUseCaseError::OrderNotFound);

        let mock_shared_state = Data::new(mock_confirm_order.clone());

        let req = TestRequest::default()
            .param("id", order_id.to_i64().to_string())
            .to_http_request();

        let resp: HttpResponse = confirm_order_endpoint(mock_shared_state, req).await;

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

        mock_confirm_order.lock_un().verify_invoked(&order_id);
    }

    #[actix_web::test]
    async fn invalid_order_state() {
        dotenv().ok();
        let order_id = rnd_order_id();
        let mock_confirm_order = AM::new_am(MockConfirmOrder::default());
        mock_confirm_order.lock_un().response = Err(ConfirmOrderUseCaseError::InvalidOrderState);

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

        mock_confirm_order.lock_un().verify_invoked(&order_id);
    }

    #[actix_web::test]
    async fn successfully_cancelled() {
        dotenv().ok();
        let order_id = rnd_order_id();
        let mock_confirm_order = AM::new_am(MockConfirmOrder::default());
        mock_confirm_order.lock_un().response = Ok(());

        let mock_shared_state = Data::new(mock_confirm_order.clone());

        let req = TestRequest::default()
            .param("id", order_id.to_i64().to_string())
            .to_http_request();

        let resp = confirm_order_endpoint(mock_shared_state, req).await;

        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        let body = resp.into_body().try_into_bytes().unwrap();

        assert!(body.is_empty());
        mock_confirm_order.lock_un().verify_invoked(&order_id);
    }
}
