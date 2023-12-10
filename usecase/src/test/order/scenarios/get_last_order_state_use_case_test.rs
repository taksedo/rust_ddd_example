use std::sync::{Arc, Mutex};

use domain::test_fixtures::{rnd_customer_id, rnd_order};

use crate::main::order::get_last_order_state::{GetLastOrderState, GetLastOrderStateUseCaseError};
use crate::main::order::scenarios::get_last_order_state_use_case::GetLastOrderStateUseCase;
use crate::test_fixtures::MockShopOrderExtractor;

#[test]
fn status_successfully_received() {
    let order = rnd_order(Default::default());
    let extractor = Arc::new(Mutex::new(MockShopOrderExtractor::default()));
    extractor.lock().unwrap().order = Some(order.clone());

    let use_case = GetLastOrderStateUseCase::new(Arc::clone(&extractor) as _);
    let result = use_case.execute(order.for_customer);

    extractor
        .lock()
        .unwrap()
        .verify_invoked_get_last_order(&order.for_customer);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), order.state)
}

#[test]
fn order_not_found() {
    let extractor = Arc::new(Mutex::new(MockShopOrderExtractor::default()));
    let use_case = GetLastOrderStateUseCase::new(Arc::clone(&extractor) as _);

    let customer_id = rnd_customer_id();
    let result = use_case.execute(customer_id);

    extractor
        .lock()
        .unwrap()
        .verify_invoked_get_last_order(&customer_id);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        GetLastOrderStateUseCaseError::OrderNotFound
    )
}
