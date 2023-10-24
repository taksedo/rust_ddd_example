use crate::main::endpoint_url::MENU_GET_BY_ID;
use crate::main::menu::get_meal_by_id_endpoint;
use crate::main::menu::meal_model::MealModel;
use crate::test_fixtures::fixtures::{rnd_meal_info, MockGetMealById, StringMethodsForRestTestExt};
use actix_web::body::MessageBody;
use actix_web::{test, web};
use domain::main::menu::value_objects::meal_id::MealId;
use std::sync::{Arc, Mutex};

#[actix_web::test]
async fn returned_successfully() {
    let meal_info = rnd_meal_info();
    let mock_get_meal_by_id = Arc::new(Mutex::new(MockGetMealById::default()));
    mock_get_meal_by_id.lock().unwrap().response = Ok(meal_info.clone());
    let mock_shared_state = web::Data::new(Arc::clone(&mock_get_meal_by_id));

    let url = MENU_GET_BY_ID
        .to_string()
        .with_id(meal_info.id.to_i64())
        .with_host();

    let req = test::TestRequest::default()
        .uri(&url)
        .param("id", meal_info.id.to_i64().clone().to_string())
        .to_http_request();

    let resp = get_meal_by_id_endpoint::execute(mock_shared_state, req).await;
    let resp = resp.unwrap();

    let body = resp.into_body().try_into_bytes().unwrap();
    let body_json = std::str::from_utf8(&body).unwrap();

    let meal_info = MealModel::from(meal_info);
    let meal_info_json = serde_json::to_string(&meal_info).unwrap();
    assert_eq!(body_json, &meal_info_json);

    mock_get_meal_by_id.lock().unwrap().verify_invoked(MealId {
        value: meal_info.id,
    });
}
