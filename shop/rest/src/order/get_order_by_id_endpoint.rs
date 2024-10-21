use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};
use common::common_rest::rest_responses::{
    get_json_from_http_response, resource_not_found, to_invalid_param_bad_request,
    GenericErrorResponse,
};
use domain::order::value_objects::shop_order_id::ShopOrderId;
use usecase::order::{
    access::shop_order_extractor::ShopOrderExtractor,
    get_order_by_id::{GetOrderById, GetOrderByIdUseCaseError},
    scenarios::get_order_by_id_use_case::GetOrderByIdUseCase,
};

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
pub async fn get_order_by_id_endpoint<T: GetOrderById + Send + Debug>(
    shared_state: web::Data<Arc<Mutex<T>>>,
    req: HttpRequest,
) -> HttpResponse {
    let id: i64 = req.match_info().get("id").unwrap().parse().unwrap();

    let error_list = Arc::new(Mutex::new(vec![]));

    match ShopOrderId::validated(id, error_list.clone()) {
        Ok(order_id) => match shared_state.lock().unwrap().execute(&order_id) {
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

#[cfg(test)]
mod tests {
    use actix_web::{body::MessageBody, http::StatusCode, test::TestRequest, web::Data};
    use common::common_rest::rest_responses::{not_found_type_url, GenericErrorResponse};
    use domain::order::shop_order::OrderState;
    use dotenvy::dotenv;

    use super::*;
    use crate::{
        domain_test_fixtures::rnd_order_id,
        test_fixtures::{rnd_order_details, MockGetOrderById},
    };

    #[actix_web::test]
    async fn order_not_found() {
        dotenv().ok();
        let order_id = rnd_order_id();
        let mock_get_order_by_id = Arc::new(Mutex::new(MockGetOrderById {
            id: rnd_order_id(),
            response: Err(GetOrderByIdUseCaseError::OrderNotFound),
        }));

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
        let item_details = details.items.get(0).unwrap();

        let mock_get_order_by_id = Arc::new(Mutex::new(MockGetOrderById {
            id: rnd_order_id(),
            response: Ok(details.clone()),
        }));

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
            response_dto.items.get(0).unwrap().meal_id,
            item_details.meal_id.to_i64()
        );
        assert_eq!(
            response_dto.items.get(0).unwrap().count,
            item_details.count.to_i32()
        );
        assert_eq!(response_dto.version, details.version.to_i64());
        mock_get_order_by_id
            .lock()
            .unwrap()
            .verify_invoked(&details.id);
    }

    #[actix_web::test]
    async fn returned_successfully_order_isnt_ready_for_confirm_or_cancel() {
        let details = rnd_order_details(OrderState::new_cancelled());
        assert_eq!(details.items.len(), 1);
        let item_details = details.items.get(0).unwrap();

        let mock_get_order_by_id = Arc::new(Mutex::new(MockGetOrderById {
            id: rnd_order_id(),
            response: Ok(details.clone()),
        }));

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
            response_dto.items.get(0).unwrap().meal_id,
            item_details.meal_id.to_i64()
        );
        assert_eq!(
            response_dto.items.get(0).unwrap().count,
            item_details.count.to_i32()
        );
        assert_eq!(response_dto.version, details.version.to_i64());

        mock_get_order_by_id
            .lock()
            .unwrap()
            .verify_invoked(&details.id);
    }
}
