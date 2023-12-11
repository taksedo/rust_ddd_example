use std::sync::{Arc, Mutex};

use domain::test_fixtures::{rnd_order, rnd_order_id};

use crate::main::order::dto::order_details::ToDetails;
use crate::main::order::get_orders::{GetOrders, GetOrdersUseCaseError};
use crate::main::order::scenarios::get_orders_use_case::GetOrdersUseCase;
use crate::test_fixtures::MockShopOrderExtractor;

#[test]
fn storage_is_empty() {
    let order_id = rnd_order_id();
    let limit: fn() -> i32 = || 10;

    let extractor = Arc::new(Mutex::new(MockShopOrderExtractor::default()));
    let use_case = GetOrdersUseCase::new(Arc::clone(&extractor) as _, limit);

    let result = use_case.execute(order_id, limit());
    let list = result.unwrap();

    assert!(list.is_empty());
    extractor.lock().unwrap().verify_invoked_get_all();
}

#[test]
fn storage_is_not_empty() {
    let limit: fn() -> i32 = || 10;

    let order = rnd_order(Default::default());
    let order_id = order.entity_params.id;

    let extractor = Arc::new(Mutex::new(MockShopOrderExtractor::default()));
    extractor.lock().unwrap().order = Some(order.clone());

    let use_case = GetOrdersUseCase::new(Arc::clone(&extractor) as _, limit);
    let result = use_case.execute(order_id, limit());
    let list = result.unwrap();

    extractor.lock().unwrap().verify_invoked_get_all();
    assert_eq!(list, vec![order.to_details()]);
}

#[test]
fn limit_exceed() {
    let limit: fn() -> i32 = || 10;
    let order_id = rnd_order_id();

    let extractor = Arc::new(Mutex::new(MockShopOrderExtractor::default()));

    let use_case = GetOrdersUseCase::new(Arc::clone(&extractor) as _, limit);
    let result = use_case.execute(order_id, limit() + 1);

    assert!(result.is_err());

    assert_eq!(result.unwrap_err(), GetOrdersUseCaseError::LimitExceed(10));
    extractor.lock().unwrap().verify_empty();
}
