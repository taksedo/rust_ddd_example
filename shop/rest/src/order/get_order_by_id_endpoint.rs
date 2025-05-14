use std::fmt::Debug;

use actix_web::{HttpRequest, HttpResponse, http::header::ContentType, web};
use common::{
    common_rest::{
        GenericErrorResponse, get_json_from_http_response, resource_not_found,
        to_invalid_param_bad_request,
    },
    types::base::{AM, AMTrait, RCell, RcRefCellTrait},
};
use domain::order::value_objects::shop_order_id::ShopOrderId;
use usecase::order::{GetOrderById, GetOrderByIdUseCaseError};

use crate::{
    endpoint_url::API_V1_ORDER_GET_BY_ID,
    order::order_model::{OrderModel, ToModel},
    to_error::ToRestError,
    validated::Validated,
};

/// Get an order by id
#[utoipa::path(
    get,
    path = API_V1_ORDER_GET_BY_ID,
    tag = "Order",
    responses(
        (
            status = OK,
            body = OrderModel,
            description = "OK" 
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
pub async fn get_order_by_id_endpoint<T>(
    shared_state: web::Data<AM<T>>,
    req: HttpRequest,
) -> HttpResponse
where
    T: GetOrderById + Send + Debug,
{
    let id: i64 = req.match_info().get("id").unwrap().parse().unwrap();

    let error_list = RCell::new_rc(vec![]);

    match ShopOrderId::validated(id, error_list.clone()) {
        Some(order_id) => match shared_state.lock_un().execute(&order_id) {
            Ok(it) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&ToModel::<OrderModel>::to_model(it)).unwrap()),
            Err(e) => e.to_rest_error(),
        },
        None => to_invalid_param_bad_request(error_list),
    }
}

impl ToRestError for GetOrderByIdUseCaseError {
    fn to_rest_error(self) -> HttpResponse {
        match self {
            GetOrderByIdUseCaseError::OrderNotFound => resource_not_found(),
        }
    }
}

pub fn get_order_by_id_endpoint_config<T>(cfg: &mut web::ServiceConfig)
where
    T: GetOrderById + 'static,
{
    cfg.route(
        API_V1_ORDER_GET_BY_ID,
        web::get().to(get_order_by_id_endpoint::<T>),
    );
}

#[cfg(test)]
mod tests {
    use actix_web::{body::MessageBody, http::StatusCode, test::TestRequest, web::Data};
    use common::common_rest::{GenericErrorResponse, not_found_type_url};
    use domain::{order::shop_order::OrderState, test_fixtures::*};
    use dotenvy::dotenv;

    use super::*;
    use crate::test_fixtures::{MockGetOrderById, rnd_order_details};

    #[actix_web::test]
    async fn order_not_found() {
        dotenv().ok();
        let order_id = rnd_order_id();
        let mock_get_order_by_id = AM::new_am(MockGetOrderById {
            id: rnd_order_id(),
            response: Err(GetOrderByIdUseCaseError::OrderNotFound),
        });

        let mock_shared_state = Data::new(mock_get_order_by_id.clone());

        let req = TestRequest::default()
            .param("id", order_id.to_i64().to_string())
            .to_http_request();

        let resp = get_order_by_id_endpoint(mock_shared_state, req).await;

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
    async fn returned_successfully_order_is_ready_for_confirm_or_cancel() {
        let details = rnd_order_details(OrderState::new_paid());
        assert_eq!(details.items.len(), 1);
        let item_details = details.items.first().unwrap();

        let mock_get_order_by_id = AM::new_am(MockGetOrderById {
            id: rnd_order_id(),
            response: Ok(details.clone()),
        });

        let mock_shared_state = Data::new(mock_get_order_by_id.clone());

        let req = TestRequest::default()
            .param("id", details.id.to_i64().to_string())
            .to_http_request();

        let resp = get_order_by_id_endpoint(mock_shared_state, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body = resp.into_body().try_into_bytes().unwrap();
        let body_text = std::str::from_utf8(&body).unwrap();

        let response_dto: OrderModel = serde_json::from_str(body_text).unwrap();

        assert_eq!(response_dto.id, details.id.to_i64());
        assert_eq!(
            response_dto.address.street,
            details.address.street_to_string()
        );
        assert_eq!(
            response_dto.address.building,
            details.address.building_to_i16()
        );
        assert_eq!(response_dto.total_price, details.total.to_string_value());
        assert_eq!(response_dto.items.len(), 1);
        assert_eq!(
            response_dto.items.first().unwrap().meal_id,
            item_details.meal_id.to_i64()
        );
        assert_eq!(
            response_dto.items.first().unwrap().count,
            item_details.count.to_i32()
        );
        assert_eq!(response_dto.version, details.version.to_i64());
        mock_get_order_by_id.lock_un().verify_invoked(&details.id);
    }

    #[actix_web::test]
    async fn returned_successfully_order_isnt_ready_for_confirm_or_cancel() {
        let details = rnd_order_details(OrderState::new_cancelled());
        assert_eq!(details.items.len(), 1);
        let item_details = details.items.first().unwrap();

        let mock_get_order_by_id = AM::new_am(MockGetOrderById {
            id: rnd_order_id(),
            response: Ok(details.clone()),
        });

        let mock_shared_state = Data::new(mock_get_order_by_id.clone());

        let req = TestRequest::default()
            .param("id", details.id.to_i64().to_string())
            .to_http_request();

        let resp = get_order_by_id_endpoint(mock_shared_state, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body = resp.into_body().try_into_bytes().unwrap();
        let body_text = std::str::from_utf8(&body).unwrap();

        let response_dto: OrderModel = serde_json::from_str(body_text).unwrap();

        assert_eq!(response_dto.id, details.id.to_i64());
        assert_eq!(
            response_dto.address.street,
            details.address.street_to_string()
        );
        assert_eq!(
            response_dto.address.building,
            details.address.building_to_i16()
        );
        assert_eq!(response_dto.total_price, details.total.to_string_value());
        assert_eq!(response_dto.items.len(), 1);
        assert_eq!(
            response_dto.items.first().unwrap().meal_id,
            item_details.meal_id.to_i64()
        );
        assert_eq!(
            response_dto.items.first().unwrap().count,
            item_details.count.to_i32()
        );
        assert_eq!(response_dto.version, details.version.to_i64());

        mock_get_order_by_id.lock_un().verify_invoked(&details.id);
    }
}
