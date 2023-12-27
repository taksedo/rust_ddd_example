use std::sync::{Arc, Mutex};

use domain::test_fixtures::rnd_order_id;

use crate::{
    main::order::{
        pay_order::{PayOrder, PayOrderHandlerError},
        scenarios::pay_order_handler::PayOrderHandler,
    },
    test_fixtures::{
        order_not_ready_for_pay, order_ready_for_pay, MockShopOrderExtractor,
        MockShopOrderPersister,
    },
};

#[test]
fn successfully_payed() {
    let order = order_ready_for_pay();
    let extractor = Arc::new(Mutex::new(MockShopOrderExtractor::default()));
    extractor.lock().unwrap().order = Some(order.clone());
    let persister = Arc::new(Mutex::new(MockShopOrderPersister::default()));

    let handler = PayOrderHandler::new(extractor.clone() as _, persister.clone() as _);
    let result = handler.execute(order.entity_params.id);

    assert!(result.is_ok());

    let order = persister.lock().unwrap().order.clone().unwrap();

    persister.lock().unwrap().verify_invoked_order(&order);
    extractor
        .lock()
        .unwrap()
        .verify_invoked_get_by_id(&order.entity_params.id);
    persister
        .lock()
        .unwrap()
        .verify_events_after_payment(&order.entity_params.id);
}

#[test]
fn invalid_state() {
    let order = order_not_ready_for_pay();
    let extractor = Arc::new(Mutex::new(MockShopOrderExtractor::default()));
    extractor.lock().unwrap().order = Some(order.clone());
    let persister = Arc::new(Mutex::new(MockShopOrderPersister::default()));

    let handler = PayOrderHandler::new(extractor.clone() as _, persister.clone() as _);
    let result = handler.execute(order.entity_params.id);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), PayOrderHandlerError::InvalidOrderState);

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

    let handler = PayOrderHandler::new(extractor.clone() as _, persister.clone() as _);
    let order_id = rnd_order_id();
    let result = handler.execute(order_id);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), PayOrderHandlerError::OrderNotFound);

    persister.lock().unwrap().verify_empty();
    extractor
        .lock()
        .unwrap()
        .verify_invoked_get_by_id(&order_id);
}
