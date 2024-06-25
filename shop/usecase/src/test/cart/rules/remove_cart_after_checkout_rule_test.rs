use std::sync::{Arc, Mutex};

use common::events::domain_event_listener::DomainEventListener;
use domain::{
    main::order::customer_order_events::{ShopOrderCreatedDomainEvent, ShopOrderEventEnum},
    test_fixtures::{rnd_cart, rnd_customer_id, rnd_order_id, rnd_price},
};
use tracing_test::traced_test;

use crate::{
    main::cart::rules::remove_cart_after_checkout_rule::RemoveCartAfterCheckoutRule,
    test_fixtures::{MockCartExtractor, MockCartRemover},
};
