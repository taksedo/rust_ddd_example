use std::sync::{Arc, Mutex};

use actix_web::{body::MessageBody, http::StatusCode, test::TestRequest, web::Data};
use common::common_rest::main::rest_responses::{bad_request_type_url, GenericErrorResponse};
use domain::test_fixtures::rnd_order_id;
use dotenvy::dotenv;
use usecase::main::order::get_orders::GetOrdersUseCaseError;

use crate::{main::order::get_orders_endpoint, test_fixtures::MockGetOrders};

#[actix_web::test]
async fn limit_reached() {
    dotenv().ok();
    let start_id = rnd_order_id();
    let limit = 10;

    let mock_get_orders = Arc::new(Mutex::new(MockGetOrders {
        response: Err(GetOrdersUseCaseError::new_limit_exceed(limit + 1)),
        start_id,
        limit,
    }));

    let mock_shared_state = Data::new(Arc::clone(&mock_get_orders));
    let req = TestRequest::default()
        .param("start_id", start_id.to_i64().to_string())
        .param("limit", limit.to_string())
        .to_http_request();

    let resp = get_orders_endpoint::execute(mock_shared_state, req).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    let body = resp.into_body().try_into_bytes().unwrap();
    let body_text = std::str::from_utf8(&body).unwrap();

    let response_dto: GenericErrorResponse = serde_json::from_str(body_text).unwrap();

    assert_eq!(&response_dto.response_type, &bad_request_type_url());
    assert_eq!(
        &response_dto.response_status,
        &StatusCode::BAD_REQUEST.as_u16()
    );
    assert_eq!(&response_dto.response_title, "Bad request");
    assert_eq!(
        &response_dto.invalid_params.get(0).unwrap().message,
        "Max limit is 10"
    );
    mock_get_orders
        .lock()
        .unwrap()
        .verify_invoked(start_id, limit);
}
