use std::sync::{Arc, Mutex};

use common::events::main::domain_event_listener::DomainEventListener;
use domain::{
    main::order::customer_order_events::{ShopOrderCreatedDomainEvent, ShopOrderEventEnum},
    test_fixtures::{rnd_customer_id, rnd_order_id, rnd_price},
};

use crate::{
    main::order::rules::export_order_after_checkout_rule::ExportOrderAfterCheckoutRule,
    test_fixtures::MockOrderExporter,
};

#[test]
fn order_has_been_exported() {
    let order_id = rnd_order_id();
    let customer_id = rnd_customer_id();
    let total_price = rnd_price();

    let exporter = Arc::new(Mutex::new(MockOrderExporter::default()));
    let mut rule = ExportOrderAfterCheckoutRule::new(exporter.clone());

    let event: ShopOrderEventEnum =
        ShopOrderCreatedDomainEvent::new(order_id, customer_id, total_price.clone()).into();

    rule.handle(&event);

    exporter
        .lock()
        .unwrap()
        .verify_invoked(order_id, customer_id, total_price);
}
