use std::sync::{Arc, Mutex};

use actix_web::{body::MessageBody, http::StatusCode, test::TestRequest, web::Data};
use common::common_rest::main::{
    cursor_paged_model::CursorPagedModel,
    rest_responses::{bad_request_type_url, GenericErrorResponse},
};
use domain::test_fixtures::rnd_order_id;
use dotenvy::dotenv;
use usecase::main::order::get_orders::GetOrdersUseCaseError;

use crate::{
    main::order::{get_orders_endpoint::get_orders_endpoint, order_model::OrderModel},
    test_fixtures::{rnd_order_details, MockGetOrders},
};

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

    let mock_shared_state = Data::new(mock_get_orders.clone());
    let req = TestRequest::default()
        .uri(&format!(
            "/?startId={}&limit={}",
            start_id.to_i64().to_string(),
            limit.to_string()
        ))
        .to_http_request();

    dbg!(&req);

    let resp = get_orders_endpoint(mock_shared_state, req).await;

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
        .verify_invoked(&start_id, &(limit + 1));
}

#[actix_web::test]
async fn returned_successfully_without_next_page() {
    dotenv().ok();
    let limit = 1;

    let single = rnd_order_details(Default::default());
    let first_item = single.clone().items[0];

    let mock_get_orders = Arc::new(Mutex::new(MockGetOrders {
        response: Ok(vec![single.clone()]),
        start_id: single.id,
        limit,
    }));

    let mock_shared_state = Data::new(mock_get_orders.clone());
    let req = TestRequest::default()
        .uri(&format!(
            "/?startId={}&limit={}",
            single.id.to_i64().to_string(),
            limit.to_string()
        ))
        .to_http_request();

    let resp = get_orders_endpoint(mock_shared_state, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let body = resp.into_body().try_into_bytes().unwrap();
    let body_text = std::str::from_utf8(&body).unwrap();

    let response_dto: CursorPagedModel<OrderModel, i32> = serde_json::from_str(body_text).unwrap();

    assert_eq!(response_dto.list.len(), limit);
    assert_eq!(response_dto.list[0].id, single.id.to_i64());
    assert_eq!(
        response_dto.list[0].total_price,
        single.total.to_string_value()
    );
    assert_eq!(response_dto.list[0].version, single.version.to_i64());
    assert_eq!(
        response_dto.list[0].address.street,
        single.address.street_to_string()
    );
    assert_eq!(
        response_dto.list[0].address.building,
        single.address.building_to_i16()
    );
    assert_eq!(response_dto.list[0].items.len(), 1);
    assert_eq!(
        response_dto.list[0].items[0].meal_id,
        first_item.meal_id.to_i64()
    );
    assert_eq!(
        response_dto.list[0].items[0].count,
        first_item.count.to_i32()
    );
    mock_get_orders
        .lock()
        .unwrap()
        .verify_invoked(&single.id, &(limit + 1));
}

#[actix_web::test]
async fn returned_successfully_with_next_page() {
    dotenv().ok();
    let limit = 1;

    let first = rnd_order_details(Default::default());
    let first_item = first.clone().items[0];
    let second = rnd_order_details(Default::default());

    let mock_get_orders = Arc::new(Mutex::new(MockGetOrders {
        response: Ok(vec![first.clone(), second]),
        start_id: first.id,
        limit,
    }));

    let mock_shared_state = Data::new(mock_get_orders.clone());
    let req = TestRequest::default()
        .uri(&format!(
            "/?startId={}&limit={}",
            first.id.to_i64().to_string(),
            limit.to_string()
        ))
        .to_http_request();

    let resp = get_orders_endpoint(mock_shared_state, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let body = resp.into_body().try_into_bytes().unwrap();
    let body_text = std::str::from_utf8(&body).unwrap();

    let response_dto: CursorPagedModel<OrderModel, i64> = serde_json::from_str(body_text).unwrap();

    dbg!(&response_dto);

    assert_eq!(response_dto.list.len(), limit);
    assert_eq!(response_dto.list[0].id, first.id.to_i64());
    assert_eq!(
        response_dto.list[0].total_price,
        first.total.to_string_value()
    );
    assert_eq!(response_dto.list[0].version, first.version.to_i64());
    assert_eq!(
        response_dto.list[0].address.street,
        first.address.street_to_string()
    );
    assert_eq!(
        response_dto.list[0].address.building,
        first.address.building_to_i16()
    );
    assert_eq!(response_dto.list[0].items.len(), 1);
    assert_eq!(
        response_dto.list[0].items[0].meal_id,
        first_item.meal_id.to_i64()
    );
    assert_eq!(
        response_dto.list[0].items[0].count,
        first_item.count.to_i32()
    );
    mock_get_orders
        .lock()
        .unwrap()
        .verify_invoked(&first.id, &(limit + 1));
}
