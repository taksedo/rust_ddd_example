#[actix_web::test]
async fn invalid_order_state() {
    dotenv().ok();
    let order_id = rnd_order_id();
    let mock_confirm_order = Arc::new(Mutex::new(MockConfirmOrder::default()));
    mock_confirm_order.lock().unwrap().response = Err(ConfirmOrderUseCaseError::InvalidOrderState);

    let mock_shared_state = Data::new(Arc::clone(&mock_confirm_order));

    let req = TestRequest::default()
        .param("id", order_id.to_i64().to_string())
        .to_http_request();

    let resp = confirm_order_endpoint::execute(mock_shared_state, req).await;

    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let body = resp.into_body().try_into_bytes().unwrap();
    let body_text = std::str::from_utf8(&body).unwrap();

    let response_dto: GenericErrorResponse = serde_json::from_str(body_text).unwrap();

    assert_eq!(
        &response_dto.response_type,
        &error_type_url("invalid_state")
    );
    assert_eq!(
        &response_dto.response_status,
        &StatusCode::UNPROCESSABLE_ENTITY.as_u16()
    );
    assert_eq!(&response_dto.response_title, "Invalid state");
}

#[actix_web::test]
async fn successfully_cancelled() {
    dotenv().ok();
    let order_id = rnd_order_id();
    let mock_confirm_order = Arc::new(Mutex::new(MockConfirmOrder::default()));
    mock_confirm_order.lock().unwrap().response = Ok(());

    let mock_shared_state = Data::new(Arc::clone(&mock_confirm_order));

    let req = TestRequest::default()
        .param("id", order_id.to_i64().to_string())
        .to_http_request();

    let resp = confirm_order_endpoint::execute(mock_shared_state, req).await;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let body = resp.into_body().try_into_bytes().unwrap();

    assert!(body.is_empty());
}
