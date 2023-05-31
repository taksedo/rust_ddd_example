use crate::main::menu::get_menu_endpoint;
use crate::main::menu::meal_model::MealModel;
use crate::test_fixtures::fixtures::{rnd_meal_info, MockGetMenu};
use actix_web::body::MessageBody;
use actix_web::web;
use std::sync::{Arc, Mutex};

#[actix_web::test]
async fn get_menu() {
    let meal_info = rnd_meal_info();
    let mock_get_menu = Arc::new(Mutex::new(MockGetMenu::default()));
    mock_get_menu.lock().unwrap().meal_info = meal_info.clone();
    let mock_shared_state = web::Data::new(Arc::clone(&mock_get_menu));

    let resp = get_menu_endpoint::execute(mock_shared_state).await;
    let resp = resp.unwrap();

    let body = resp.into_body().try_into_bytes().unwrap();
    let body_text = std::str::from_utf8(&body).unwrap();
    let body_text = body_text
        .replace("MealModel", "")
        .replace("id", "\"id\"")
        .replace("name", "\"name\"")
        .replace("description", "\"description\"")
        .replace("price", "\"price\"")
        .replace("version", "\"version\"");

    let list_of_meal_model_from_resp: Vec<MealModel> =
        serde_json::from_str(&body_text.replace("MealModel", "")).unwrap();

    assert_eq!(list_of_meal_model_from_resp.len(), 1);
    assert_eq!(
        list_of_meal_model_from_resp.get(0).unwrap().id,
        meal_info.id.to_u64()
    );
    assert_eq!(
        list_of_meal_model_from_resp.get(0).unwrap().name,
        meal_info.name.to_string_value()
    );
    assert_eq!(
        list_of_meal_model_from_resp.get(0).unwrap().description,
        meal_info.description.to_string_value()
    );
    assert_eq!(
        list_of_meal_model_from_resp.get(0).unwrap().price,
        meal_info.price.to_f64_value()
    );
    assert_eq!(
        list_of_meal_model_from_resp.get(0).unwrap().version,
        meal_info.version.to_u64()
    );
}
