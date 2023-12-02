use std::sync::{Arc, Mutex};

use common::events::main::domain_event_listener::DomainEventListener;

use domain::main::order::customer_order_events::{ShopOrderCreatedDomainEvent, ShopOrderEventEnum};
use domain::test_fixtures::{rnd_cart, rnd_customer_id, rnd_order_id, rnd_price};

use crate::main::cart::rules::remove_cart_after_checkout_rule::RemoveCartAfterCheckoutRule;
use crate::test_fixtures::{MockCartExtractor, MockCartRemover};

#[test]
fn successfully_removed() {
    let cart_remover = Arc::new(Mutex::new(MockCartRemover::default()));
    let cart = rnd_cart();

    let cart_extractor = Arc::new(Mutex::new(MockCartExtractor::default()));
    cart_extractor.lock().unwrap().cart = Some(cart.clone());

    let mut rule = RemoveCartAfterCheckoutRule::new(
        Arc::clone(&cart_extractor) as _,
        Arc::clone(&cart_remover) as _,
    );
    let event: ShopOrderEventEnum =
        ShopOrderCreatedDomainEvent::new(rnd_order_id(), cart.clone().for_customer, rnd_price())
            .into();

    rule.handle(&event);

    cart_extractor
        .lock()
        .unwrap()
        .verify_invoked(Some(cart.clone().for_customer));
    cart_remover
        .lock()
        .unwrap()
        .verify_invoked(cart.clone().entity_param.id);
}

#[test]
fn cart_not_found() {
    let cart_remover = Arc::new(Mutex::new(MockCartRemover::default()));

    let cart_extractor = Arc::new(Mutex::new(MockCartExtractor::default()));

    let mut rule = RemoveCartAfterCheckoutRule::new(
        Arc::clone(&cart_extractor) as _,
        Arc::clone(&cart_remover) as _,
    );
    let customer_id = rnd_customer_id();
    let event: ShopOrderEventEnum =
        ShopOrderCreatedDomainEvent::new(rnd_order_id(), customer_id, rnd_price()).into();

    rule.handle(&event);

    cart_extractor
        .lock()
        .unwrap()
        .verify_invoked(Some(customer_id));
    cart_remover.lock().unwrap().verify_empty();
}
