use crate::main::endpoint_url::API_V1_MENU_GET_BY_ID;
use crate::main::menu::add_meal_to_menu_endpoint;
use crate::main::menu::add_meal_to_menu_endpoint::MealStruct;
use crate::test_fixtures::fixtures::MockAddMealToMenu;
use actix_web::body::MessageBody;
use actix_web::http::{header, StatusCode};
use actix_web::{web, web::Json};
use bigdecimal::num_bigint::BigInt;
use bigdecimal::BigDecimal;
use common_rest::main::rest_responses::GenericErrorResponse;
use domain::test_fixtures::fixtures::{
    rnd_meal_description, rnd_meal_id, rnd_meal_name, rnd_price,
};
use dotenvy::dotenv;
use std::sync::{Arc, Mutex};

#[actix_web::test]
async fn created_successfully() {
    dotenv().ok();
    let meal_id = rnd_meal_id();
    let meal_name = rnd_meal_name();
    let meal_description = rnd_meal_description();
    let price = rnd_price();

    let mock_add_meal_to_menu = Arc::new(Mutex::new(MockAddMealToMenu::default()));
    mock_add_meal_to_menu.lock().unwrap().response = Ok(meal_id);

    let mock_shared_state = web::Data::new(Arc::clone(&mock_add_meal_to_menu));

    let meal = Json(MealStruct::new(
        meal_name.clone().to_string(),
        meal_description.clone().to_string(),
        price.clone().to_bigdecimal(),
    ));

    let resp = add_meal_to_menu_endpoint::execute(mock_shared_state, meal).await;

    mock_add_meal_to_menu
        .lock()
        .unwrap()
        .verify_invoked(meal_name, meal_description, price);

    let header = resp
        .headers()
        .get(header::LOCATION)
        .unwrap()
        .to_str()
        .unwrap();

    assert_eq!(&resp.status(), &StatusCode::CREATED);
    assert_eq!(
        header,
        API_V1_MENU_GET_BY_ID.replace("{id}", meal_id.value.to_string().as_str())
    );
}

#[actix_web::test]
async fn validation_error() {
    dotenv().ok();
    let mock_add_meal_to_menu = Arc::new(Mutex::new(MockAddMealToMenu::default()));
    let mock_shared_state = web::Data::new(Arc::clone(&mock_add_meal_to_menu));

    let meal = Json(MealStruct::new(
        "".to_string(),
        "".to_string(),
        BigDecimal::new(BigInt::from(1), 20),
    ));

    let resp = add_meal_to_menu_endpoint::execute(mock_shared_state as _, meal).await;
    let body = resp.into_body().try_into_bytes().unwrap();
    let body_text = std::str::from_utf8(&body).unwrap();

    let response_dto: GenericErrorResponse = serde_json::from_str(body_text).unwrap();

    assert_eq!(
        &response_dto.response_status,
        &StatusCode::BAD_REQUEST.as_u16()
    );
    assert_eq!(&response_dto.response_title, "Bad request");
    assert_eq!(&response_dto.invalid_params.iter().count(), &3);
}
