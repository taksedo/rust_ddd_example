use crate::main::menu::add_meal_to_menu_endpoint;
use crate::main::menu::add_meal_to_menu_endpoint::MealStruct;
use crate::test_fixtures::fixtures::MockAddMealToMenu;
use actix_web::http::{header, StatusCode};
use actix_web::{web, web::Json};
use bigdecimal::{BigDecimal, One};
use domain::test_fixtures::fixtures::{
    rnd_meal_description, rnd_meal_id, rnd_meal_name, rnd_price,
};
use std::sync::{Arc, Mutex};

#[actix_web::test]
async fn created_successfully() {
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
        price.clone().to_bigdecimal_value(),
    ));

    let resp = add_meal_to_menu_endpoint::execute(mock_shared_state, meal).await;

    mock_add_meal_to_menu
        .lock()
        .unwrap()
        .verify_invoked(meal_name, meal_description, price);
    let resp = resp.unwrap();

    let header = resp
        .headers()
        .get(header::LOCATION)
        .unwrap()
        .to_str()
        .unwrap();

    assert_eq!(&resp.status(), &StatusCode::OK);
    assert_eq!(header, &meal_id.to_i64().to_string());
}

#[actix_web::test]
async fn validation_error() {
    let mock_add_meal_to_menu = Arc::new(Mutex::new(MockAddMealToMenu::default()));
    let mock_shared_state = web::Data::new(Arc::clone(&mock_add_meal_to_menu));

    let meal = Json(MealStruct::new(
        "".to_string(),
        "".to_string(),
        BigDecimal::one().with_scale(20),
    ));

    let resp = add_meal_to_menu_endpoint::execute(mock_shared_state as _, meal).await;
    let resp = resp.expect_err("Обнаружена ошибка");

    assert_eq!(
        resp.as_response_error().status_code(),
        StatusCode::INTERNAL_SERVER_ERROR
    );
}
