use std::sync::{Arc, Mutex};

use domain::test_fixtures::{rnd_order, rnd_order_id};

use crate::main::order::get_last_order_state::GetLastOrderState;
use crate::main::order::get_order_by_id::{GetOrderById, GetOrderByIdUseCaseError};
use crate::main::order::scenarios::get_last_order_state_use_case::GetLastOrderStateUseCase;
use crate::main::order::scenarios::get_order_by_id_use_case::GetOrderByIdUseCase;
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
    let use_case = GetOrderByIdUseCase::new(Arc::clone(&extractor) as _);

    let order_id = rnd_order_id();
    let result = use_case.execute(order_id);

    extractor
        .lock()
        .unwrap()
        .verify_invoked_get_by_id(&order_id);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), GetOrderByIdUseCaseError::OrderNotFound)
}

#[test]
fn order_expected_successfully() {
    let order = rnd_order(Default::default());
    let extractor = Arc::new(Mutex::new(MockShopOrderExtractor::default()));
    extractor.lock().unwrap().order = Some(order.clone());
    let use_case = GetOrderByIdUseCase::new(Arc::clone(&extractor) as _);

    let result = use_case.execute(order.entity_params.id);
    assert!(result.is_ok());
    let details = result.unwrap();

    assert_eq!(details.id, order.entity_params.id);
    assert_eq!(details.address, order.address);
    assert_eq!(details.state, order.state);
    assert_eq!(details.total, order.total_price());
    assert_eq!(
        details.ready_for_confirm_or_cancel,
        order.ready_for_confirm_or_cancel()
    );
    assert_eq!(details.items.len(), order.order_items.len());

    details.items.iter().for_each(|i| {
        let src_item: Vec<_> = order
            .order_items
            .iter()
            .filter(|&it| it.meal_id == i.meal_id && it.count == i.count)
            .collect();
        assert_eq!(src_item.len(), 1);
    });
    extractor
        .lock()
        .unwrap()
        .verify_invoked_get_by_id(&order.entity_params.id);
}
