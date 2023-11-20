#[actix_web::test]
async fn meal_not_found() {
    dotenv().ok();
    let meal_id = rnd_meal_id();
    let mock_remove_meal_from_menu = Arc::new(Mutex::new(MockRemoveMealFromMenu::default()));
    mock_remove_meal_from_menu.lock().unwrap().response =
        Err(RemoveMealFromMenuUseCaseError::MealNotFound);
    let mock_shared_state = web::Data::new(Arc::clone(&mock_remove_meal_from_menu));

    let url = API_V1_MENU_DELETE_BY_ID
        .to_string()
        .with_id(meal_id.to_i64())
        .with_host();

    let req = test::TestRequest::default()
        .uri(&url)
        .param("id", meal_id.to_i64().clone().to_string())
        .to_http_request();

    let resp = remove_meal_from_menu_endpoint::execute(mock_shared_state, req).await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn removed_successfully() {
    let meal_id = rnd_meal_id();

    let mock_remove_meal_from_menu = Arc::new(Mutex::new(MockRemoveMealFromMenu::default()));
    let mock_shared_state = web::Data::new(Arc::clone(&mock_remove_meal_from_menu));

    let url = API_V1_MENU_DELETE_BY_ID
        .to_string()
        .with_id(meal_id.to_i64())
        .with_host();

    let req = test::TestRequest::default()
        .uri(&url)
        .param("id", meal_id.to_i64().clone().to_string())
        .to_http_request();

    let resp = remove_meal_from_menu_endpoint::execute(mock_shared_state, req).await;

    let body = resp.into_body().try_into_bytes().unwrap();

    assert!(body.is_empty());
}
