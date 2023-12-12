#![allow(unused_imports)]

use std::sync::{Arc, Mutex};

use actix_web::body::MessageBody;
use actix_web::http::StatusCode;
use actix_web::{test, web};
use common::common_rest::main::rest_responses::{
    error_type_url, not_found_type_url, GenericErrorResponse,
};
use dotenvy::dotenv;

use domain::test_fixtures::rnd_order_id;
use usecase::main::order::cancel_order::CancelOrderUseCaseError;

use crate::main::endpoint_url::API_V1_ORDER_CANCEL_BY_ID;
use crate::main::menu::remove_meal_from_menu_endpoint;
use crate::main::order::cancel_order_endpoint;
use crate::test_fixtures::MockCancelOrder;
use crate::test_fixtures::StringMethodsForRestTestExt;

#[actix_web::test]
async fn order_not_found() {
    dotenv().ok();
    let order_id = rnd_order_id();
    let mock_cancel_order = Arc::new(Mutex::new(MockCancelOrder::default()));
    mock_cancel_order.lock().unwrap().response = Err(CancelOrderUseCaseError::OrderNotFound);

    let mock_shared_state = web::Data::new(Arc::clone(&mock_cancel_order));

    let url = API_V1_ORDER_CANCEL_BY_ID
        .to_string()
        .with_id(&order_id.to_i64())
        .with_host();

    let req = test::TestRequest::default()
        .uri(&url)
        .param("id", order_id.to_i64().clone().to_string())
        .to_http_request();

    let resp = cancel_order_endpoint::execute(mock_shared_state, req).await;

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
    mock_cancel_order.lock().unwrap().response = Err(CancelOrderUseCaseError::InvalidOrderState);

    let mock_shared_state = web::Data::new(Arc::clone(&mock_cancel_order));

    let url = API_V1_ORDER_CANCEL_BY_ID
        .to_string()
        .with_id(&order_id.to_i64())
        .with_host();

    let req = test::TestRequest::default()
        .uri(&url)
        .param("id", order_id.to_i64().clone().to_string())
        .to_http_request();

    let resp = cancel_order_endpoint::execute(mock_shared_state, req).await;

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

    let mock_shared_state = web::Data::new(Arc::clone(&mock_cancel_order));

    let url = API_V1_ORDER_CANCEL_BY_ID
        .to_string()
        .with_id(&order_id.to_i64())
        .with_host();

    let req = test::TestRequest::default()
        .uri(&url)
        .param("id", order_id.to_i64().clone().to_string())
        .to_http_request();

    let resp = cancel_order_endpoint::execute(mock_shared_state, req).await;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let body = resp.into_body().try_into_bytes().unwrap();

    assert!(body.is_empty());
}
