use crate::main::endpoint_url::MENU_DELETE_BY_ID;
use crate::main::menu::remove_meal_from_menu_endpoint;
use crate::test_fixtures::fixtures::{MockRemoveMealFromMenu, StringMethodsForRestTestExt};
use actix_web::body::MessageBody;
use actix_web::http::StatusCode;
use actix_web::{test, web};
use domain::test_fixtures::fixtures::rnd_meal_id;
use std::sync::{Arc, Mutex};
use usecase::main::menu::remove_meal_from_menu::RemoveMealFromMenuUseCaseError;

#[actix_web::test]
async fn meal_not_found() {
    let meal_id = rnd_meal_id();
    let mock_remove_meal_from_menu = Arc::new(Mutex::new(MockRemoveMealFromMenu::default()));
    mock_remove_meal_from_menu.lock().unwrap().response =
        Err(RemoveMealFromMenuUseCaseError::MealNotFound);
    let mock_shared_state = web::Data::new(Arc::clone(&mock_remove_meal_from_menu));

    let url = MENU_DELETE_BY_ID
        .to_string()
        .with_id(meal_id.to_i64())
        .with_host();

    let req = test::TestRequest::default()
        .uri(&url)
        .param("id", meal_id.to_i64().clone().to_string())
        .to_http_request();

    let resp = remove_meal_from_menu_endpoint::execute(mock_shared_state, req).await;

    let resp = resp.expect_err("Обнаружена ошибка");

    assert_eq!(
        resp.as_response_error().status_code(),
        StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[actix_web::test]
async fn removed_successfully() {
    let meal_id = rnd_meal_id();

    let mock_remove_meal_from_menu = Arc::new(Mutex::new(MockRemoveMealFromMenu::default()));
    let mock_shared_state = web::Data::new(Arc::clone(&mock_remove_meal_from_menu));

    let url = MENU_DELETE_BY_ID
        .to_string()
        .with_id(meal_id.to_i64())
        .with_host();

    let req = test::TestRequest::default()
        .uri(&url)
        .param("id", meal_id.to_i64().clone().to_string())
        .to_http_request();

    let resp = remove_meal_from_menu_endpoint::execute(mock_shared_state, req)
        .await
        .unwrap();

    let resp = resp;

    let body = resp.into_body().try_into_bytes().unwrap();

    assert!(body.is_empty());
}
