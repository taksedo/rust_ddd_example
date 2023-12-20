use std::sync::{Arc, Mutex};

use actix_web::{body::MessageBody, http::StatusCode, test::TestRequest, web::Data};
use common::common_rest::main::rest_responses::{not_found_type_url, GenericErrorResponse};
use domain::{main::order::shop_order::OrderState, test_fixtures::rnd_order_id};
use dotenvy::dotenv;
use usecase::main::order::get_order_by_id::GetOrderByIdUseCaseError;

use crate::{
    main::order::{get_order_by_id_endpoint, order_model::OrderModel},
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

    let resp = get_order_by_id_endpoint::execute(mock_shared_state, req).await;

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

    let resp = get_order_by_id_endpoint::execute(mock_shared_state, req).await;

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

    let resp = get_order_by_id_endpoint::execute(mock_shared_state, req).await;

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
