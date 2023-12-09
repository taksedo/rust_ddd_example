use std::sync::{Arc, Mutex};

use crate::main::order::cancel_order::CancelOrder;
use crate::main::order::scenarios::cancel_order_use_case::CancelOrderUseCase;
use crate::test_fixtures::{
    order_ready_for_cancel, MockShopOrderExtractor, MockShopOrderPersister,
};

#[test]
fn successfully_confirmed() {
    let order = order_ready_for_cancel();
    // let mut pinned_order = Cell::new(order);

    // let order_1 = pinned_order.get_mut();

    let extractor = Arc::new(Mutex::new(MockShopOrderExtractor::default()));
    let persister = Arc::new(Mutex::new(MockShopOrderPersister::default()));
    extractor.lock().unwrap().order = Some(order.clone());

    let use_case =
        CancelOrderUseCase::new(Arc::clone(&extractor) as _, Arc::clone(&persister) as _);
    let result = use_case.execute(order.entity_params.id);
    dbg!(&use_case);

    assert!(result.is_ok());

    let order = &persister.lock().unwrap().order.clone().unwrap();
    assert_eq!(order, &persister.lock().unwrap().order.clone().unwrap());
    persister.lock().unwrap().verify_invoked_order(order);
    persister
        .lock()
        .unwrap()
        .verify_events_after_cancellation(&order.entity_params.id);
    extractor
        .lock()
        .unwrap()
        .verify_invoked_get_by_id(&order.entity_params.id);

    println!("FIRST: {:?}, \n\nSECOND: {:?}", order, extractor);
}
