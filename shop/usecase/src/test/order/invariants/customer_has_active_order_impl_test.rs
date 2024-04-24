use std::sync::{Arc, Mutex};

use domain::{
    main::order::customer_has_active_order::CustomerHasActiveOrder, test_fixtures::rnd_customer_id,
};

use crate::{
    main::order::invariants::customer_has_active_order_impl::CustomerHasActiveOrderImpl,
    test_fixtures::{active_order, non_active_order, MockShopOrderExtractor},
};

#[test]
fn active_order_exists() {
    let active_order = active_order();
    let extractor = Arc::new(Mutex::new(MockShopOrderExtractor {
        order: Some(active_order.clone()),
        ..Default::default()
    }));
    let mut rule = CustomerHasActiveOrderImpl::new(extractor.clone());

    let has_active_order = rule.invoke(active_order.for_customer);

    assert!(has_active_order);
    extractor
        .lock()
        .unwrap()
        .verify_invoked_get_last_order(&active_order.for_customer);
}

#[test]
fn order_exists_but_not_active() {
    let active_order = non_active_order();
    let extractor = Arc::new(Mutex::new(MockShopOrderExtractor {
        order: Some(active_order.clone()),
        ..Default::default()
    }));
    let mut rule = CustomerHasActiveOrderImpl::new(extractor.clone());

    let has_active_order = rule.invoke(active_order.for_customer);

    assert!(!has_active_order);
    extractor
        .lock()
        .unwrap()
        .verify_invoked_get_last_order(&active_order.for_customer);
}

#[test]
fn order_doesnt_exist() {
    let extractor = Arc::new(Mutex::new(MockShopOrderExtractor::default()));
    let mut rule = CustomerHasActiveOrderImpl::new(extractor.clone());

    let customer_id = rnd_customer_id();
    let has_active_order = rule.invoke(customer_id);

    assert!(!has_active_order);
    extractor
        .lock()
        .unwrap()
        .verify_invoked_get_last_order(&customer_id);
}
