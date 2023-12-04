use crate::main::order::shop_order::OrderState;
use crate::main::order::shop_order_restorer::ShopOrderRestorer;
use crate::test_fixtures::{rnd_address, rnd_customer_id, rnd_order_id, rnd_order_item, rnd_price};
use common::types::main::base::domain_entity::{DomainEntityTrait, Version};
use common::types::test_fixtures::rnd_count;
use std::collections::HashSet;
use time::OffsetDateTime;

#[test]
fn restore_user_success() {
    let id = rnd_order_id();
    let created = OffsetDateTime::now_utc();
    let customer_id = rnd_customer_id();
    let item = rnd_order_item(rnd_price(), rnd_count());
    let items = HashSet::from([item.clone()]);
    let state = OrderState::new_completed();
    let version = Version::default();
    let address = rnd_address();

    let mut order = ShopOrderRestorer::restore_order(
        id,
        created,
        customer_id,
        address.clone(),
        items.clone(),
        state.clone(),
        version,
    );

    assert_eq!(order.entity_params.id, id);
    assert_eq!(order.created, created);
    assert_eq!(order.for_customer, customer_id);
    assert_eq!(order.address, address);
    assert_eq!(order.order_items.len(), 1);
    let order_item = order.order_items.iter().next().unwrap().clone();
    assert_eq!(order_item.price, item.price);
    assert_eq!(order_item.meal_id, item.meal_id);
    assert_eq!(order_item.count, item.count);

    assert_eq!(order.state, state);
    assert_eq!(order.entity_params.version, version);
    assert!(order.entity_params.pop_events().is_empty());
}
