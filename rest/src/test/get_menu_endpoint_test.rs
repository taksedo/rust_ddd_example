#[actix_web::test]
async fn get_menu() {
    let meal_info = rnd_meal_info();
    let mock_get_menu = Arc::new(Mutex::new(MockGetMenu::default()));
    mock_get_menu.lock().unwrap().meal_info = meal_info.clone();
    let mock_shared_state = web::Data::new(Arc::clone(&mock_get_menu));

    let resp = get_menu_endpoint::execute(mock_shared_state).await;

    let body = resp.into_body().try_into_bytes().unwrap();
    let body_text = std::str::from_utf8(&body).unwrap();

    let list_of_meal_model_from_resp: Vec<MealModel> = serde_json::from_str(body_text).unwrap();

    assert_eq!(list_of_meal_model_from_resp.len(), 1);
    assert_eq!(
        list_of_meal_model_from_resp.get(0).unwrap().id,
        meal_info.id.to_i64()
    );
    assert_eq!(
        list_of_meal_model_from_resp.get(0).unwrap().name,
        meal_info.name.to_string()
    );
    assert_eq!(
        list_of_meal_model_from_resp.get(0).unwrap().description,
        meal_info.description.to_string()
    );
    assert_eq!(
        list_of_meal_model_from_resp.get(0).unwrap().price,
        meal_info.price.to_bigdecimal()
    );
    assert_eq!(
        list_of_meal_model_from_resp.get(0).unwrap().version,
        meal_info.version.to_i64()
    );
}
