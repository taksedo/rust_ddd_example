use std::sync::{Arc, Mutex};

use actix_web::{
    body::MessageBody,
    http::{header, StatusCode},
    web::{Data, Json},
};
use bigdecimal::{num_bigint::BigInt, BigDecimal, ToPrimitive};
use common::common_rest::rest_responses::{
    bad_request_type_url, error_type_url, GenericErrorResponse,
};
use domain::test_fixtures::{rnd_meal_description, rnd_meal_id, rnd_meal_name, rnd_price};
use dotenvy::dotenv;
use usecase::main::menu::add_meal_to_menu::AddMealToMenuUseCaseError;

use crate::{
    main::{
        endpoint_url::API_V1_MENU_GET_BY_ID,
        menu::{add_meal_to_menu_endpoint, add_meal_to_menu_endpoint::AddMealToMenuRestRequest},
    },
    test::menu::add_meal_to_menu_endpoint_test::add_meal_to_menu_endpoint::add_meal_to_menu_endpoint,
    test_fixtures::{MockAddMealToMenu, StringMethodsForRestTestExt},
};

#[actix_web::test]
async fn created_successfully() {
    dotenv().ok();
    let meal_id = rnd_meal_id();
    let meal_name = rnd_meal_name();
    let meal_description = rnd_meal_description();
    let price = rnd_price();

    let mock_add_meal_to_menu = mock_add_meal_to_menu();
    mock_add_meal_to_menu.lock().unwrap().response = Ok(meal_id);

    let mock_shared_state = mock_shared_state(&mock_add_meal_to_menu);

    let meal = Json(AddMealToMenuRestRequest::new(
        meal_name.clone().to_string(),
        meal_description.clone().to_string(),
        price.to_bigdecimal().to_f64().unwrap(),
    ));

    let resp = add_meal_to_menu_endpoint(mock_shared_state, meal).await;

    mock_add_meal_to_menu
        .lock()
        .unwrap()
        .verify_invoked(&meal_name, &meal_description, &price);

    let header = resp
        .headers()
        .get(header::LOCATION)
        .unwrap()
        .to_str()
        .unwrap();

    assert_eq!(&resp.status(), &StatusCode::CREATED);
    assert_eq!(
        header,
        API_V1_MENU_GET_BY_ID.to_string().with_id(&meal_id.to_i64())
    );
}

#[actix_web::test]
async fn validation_error() {
    dotenv().ok();
    let mock_add_meal_to_menu = mock_add_meal_to_menu();
    let mock_shared_state = mock_shared_state(&mock_add_meal_to_menu);

    let meal = Json(AddMealToMenuRestRequest::new(
        "".to_string(),
        "".to_string(),
        BigDecimal::new(BigInt::from(1), 20).to_f64().unwrap(),
    ));

    let resp = add_meal_to_menu_endpoint(mock_shared_state, meal).await;
    let body = resp.into_body().try_into_bytes().unwrap();
    let body_text = std::str::from_utf8(&body).unwrap();

    let response_dto: GenericErrorResponse = serde_json::from_str(body_text).unwrap();

    assert_eq!(
        &response_dto.response_status,
        &StatusCode::BAD_REQUEST.as_u16()
    );
    assert_eq!(&response_dto.response_type, &bad_request_type_url());
    assert_eq!(
        &response_dto.response_status,
        &StatusCode::BAD_REQUEST.as_u16()
    );
    assert_eq!(&response_dto.response_title, &"Bad request");
    assert_eq!(response_dto.invalid_params.len(), 3);
}

#[actix_web::test]
async fn meal_already_exists() {
    dotenv().ok();
    let mock_add_meal_to_menu = mock_add_meal_to_menu();
    let mock_shared_state = mock_shared_state(&mock_add_meal_to_menu);
    mock_add_meal_to_menu.lock().unwrap().response = Err(AddMealToMenuUseCaseError::AlreadyExists);
    let meal = Json(AddMealToMenuRestRequest::new(
        rnd_meal_name().to_string(),
        rnd_meal_description().to_string(),
        rnd_price().to_f64(),
    ));

    let resp = add_meal_to_menu_endpoint(mock_shared_state, meal).await;

    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let body = resp.into_body().try_into_bytes().unwrap();
    let body_text = std::str::from_utf8(&body).unwrap();

    let response_dto: GenericErrorResponse = serde_json::from_str(body_text).unwrap();

    assert_eq!(
        &response_dto.response_status,
        &StatusCode::UNPROCESSABLE_ENTITY.as_u16()
    );
    assert_eq!(
        &response_dto.response_type,
        &error_type_url("already_exists")
    );
    assert_eq!(
        &response_dto.response_status,
        &StatusCode::UNPROCESSABLE_ENTITY.as_u16()
    );
    assert_eq!(&response_dto.response_title, "Meal already exists");
}

fn mock_shared_state(
    mock_add_meal_to_menu: &Arc<Mutex<MockAddMealToMenu>>,
) -> Data<Arc<Mutex<MockAddMealToMenu>>> {
    Data::new(Arc::clone(mock_add_meal_to_menu))
}

fn mock_add_meal_to_menu() -> Arc<Mutex<MockAddMealToMenu>> {
    Arc::new(Mutex::new(MockAddMealToMenu::default()))
}
