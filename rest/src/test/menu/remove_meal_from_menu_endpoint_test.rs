use std::sync::{Arc, Mutex};

use actix_web::{body::MessageBody, http::StatusCode, test::TestRequest, web::Data};
use common::common_rest::main::rest_responses::{not_found_type_url, GenericErrorResponse};
use domain::test_fixtures::rnd_meal_id;
use dotenvy::dotenv;
use usecase::main::menu::remove_meal_from_menu::RemoveMealFromMenuUseCaseError;

use crate::{main::menu::remove_meal_from_menu_endpoint, test_fixtures::MockRemoveMealFromMenu};

#[actix_web::test]
async fn meal_not_found() {
    dotenv().ok();
    let meal_id = rnd_meal_id();
    let mock_remove_meal_from_menu = Arc::new(Mutex::new(MockRemoveMealFromMenu::default()));
    mock_remove_meal_from_menu.lock().unwrap().response =
        Err(RemoveMealFromMenuUseCaseError::MealNotFound);
    let mock_shared_state = Data::new(Arc::clone(&mock_remove_meal_from_menu));

    let req = TestRequest::default()
        .param("id", meal_id.to_i64().to_string())
        .to_http_request();

    let resp = remove_meal_from_menu_endpoint::execute(mock_shared_state, req).await;

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
async fn removed_successfully() {
    let meal_id = rnd_meal_id();

    let mock_remove_meal_from_menu = Arc::new(Mutex::new(MockRemoveMealFromMenu::default()));
    let mock_shared_state = Data::new(Arc::clone(&mock_remove_meal_from_menu));

    let req = TestRequest::default()
        .param("id", meal_id.to_i64().to_string())
        .to_http_request();

    let resp = remove_meal_from_menu_endpoint::execute(mock_shared_state, req).await;

    let body = resp.into_body().try_into_bytes().unwrap();

    assert!(body.is_empty());
}
