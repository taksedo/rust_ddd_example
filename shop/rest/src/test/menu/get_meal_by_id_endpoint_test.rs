use std::sync::{Arc, Mutex};

use actix_web::{body::MessageBody, http::StatusCode, test::TestRequest, web::Data};
use common::common_rest::rest_responses::{not_found_type_url, GenericErrorResponse};
use domain::{main::menu::value_objects::meal_id::MealId, test_fixtures::rnd_meal_id};
use dotenvy::dotenv;
use usecase::main::menu::get_meal_by_id::GetMealByIdUseCaseError::MealNotFound;

use crate::{
    main::menu::{get_meal_by_id_endpoint::get_meal_by_id_endpoint, meal_model::MealModel},
    test_fixtures::{rnd_meal_info, MockGetMealById},
};

#[actix_web::test]
async fn returned_successfully() {
    let meal_info = rnd_meal_info();

    let mock_get_meal_by_id = mock_get_meal_by_id();
    let mock_shared_state = mock_shared_state(&mock_get_meal_by_id);

    mock_get_meal_by_id.lock().unwrap().response = Ok(meal_info.clone());

    let req = TestRequest::default()
        .param("id", meal_info.id.to_i64().to_string())
        .to_http_request();

    let resp = get_meal_by_id_endpoint(mock_shared_state, req).await;

    let body = resp.into_body().try_into_bytes().unwrap();
    let body_json = std::str::from_utf8(&body).unwrap();

    let meal_info = MealModel::from(meal_info);
    let meal_info_json = serde_json::to_string(&meal_info).unwrap();
    assert_eq!(body_json, &meal_info_json);

    mock_get_meal_by_id
        .lock()
        .unwrap()
        .verify_invoked(MealId::try_from(meal_info.id).unwrap());
}

#[actix_web::test]
async fn meal_not_found() {
    dotenv().ok();
    let mock_get_meal_by_id = mock_get_meal_by_id();
    let mock_shared_state = mock_shared_state(&mock_get_meal_by_id);

    mock_get_meal_by_id.lock().unwrap().response = Err(MealNotFound);

    let meal_id = rnd_meal_id().to_i64();

    let req = TestRequest::default()
        .param("id", meal_id.to_string())
        .to_http_request();

    let resp = get_meal_by_id_endpoint(mock_shared_state, req).await;

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

fn mock_get_meal_by_id() -> Arc<Mutex<MockGetMealById>> {
    Arc::new(Mutex::new(MockGetMealById::default()))
}

fn mock_shared_state(
    mock_get_meal_by_id: &Arc<Mutex<MockGetMealById>>,
) -> Data<Arc<Mutex<MockGetMealById>>> {
    Data::new(Arc::clone(mock_get_meal_by_id))
}
