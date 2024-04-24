use std::sync::{Arc, Mutex};

use domain::test_fixtures::rnd_order_id;

use crate::{
    main::order::{
        complete_order::{CompleteOrder, CompleteOrderUseCaseError},
        scenarios::complete_order_use_case::CompleteOrderUseCase,
    },
    test_fixtures::{
        order_not_ready_for_complete, order_ready_for_complete, MockShopOrderExtractor,
        MockShopOrderPersister,
    },
};

#[test]
fn successfully_completed() {
    let order = order_ready_for_complete();
    let extractor = Arc::new(Mutex::new(MockShopOrderExtractor::default()));
    extractor.lock().unwrap().order = Some(order.clone());
    let persister = Arc::new(Mutex::new(MockShopOrderPersister::default()));

    let use_case = CompleteOrderUseCase::new(extractor.clone(), persister.clone());
    let result = use_case.execute(order.entity_params.id);

    assert!(result.is_ok());

    let order = persister.lock().unwrap().order.clone().unwrap();
    persister.lock().unwrap().verify_invoked_order(&order);
    persister
        .lock()
        .unwrap()
        .verify_events_after_completion(&order.entity_params.id);
    extractor
        .lock()
        .unwrap()
        .verify_invoked_get_by_id(&order.entity_params.id);
}

#[test]
fn invalid_state() {
    let order = order_not_ready_for_complete();
    let extractor = Arc::new(Mutex::new(MockShopOrderExtractor::default()));
    extractor.lock().unwrap().order = Some(order.clone());
    let persister = Arc::new(Mutex::new(MockShopOrderPersister::default()));

    let use_case = CompleteOrderUseCase::new(extractor.clone(), persister.clone());
    let result = use_case.execute(order.entity_params.id);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        CompleteOrderUseCaseError::InvalidOrderState
    );

    persister.lock().unwrap().verify_empty();
    extractor
        .lock()
        .unwrap()
        .verify_invoked_get_by_id(&order.entity_params.id);
}

#[test]
fn order_not_found() {
    let extractor = Arc::new(Mutex::new(MockShopOrderExtractor::default()));
    let persister = Arc::new(Mutex::new(MockShopOrderPersister::default()));

    let use_case = CompleteOrderUseCase::new(extractor.clone(), persister.clone());

    let order_id = rnd_order_id();
    let result = use_case.execute(order_id);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        CompleteOrderUseCaseError::OrderNotFound
    );

    persister.lock().unwrap().verify_empty();
    extractor
        .lock()
        .unwrap()
        .verify_invoked_get_by_id(&order_id);
}
