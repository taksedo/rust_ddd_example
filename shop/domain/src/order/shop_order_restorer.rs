use std::collections::HashSet;

use common::types::{
    base::{DomainEntity, Version},
    common::Address,
};
use time::OffsetDateTime;

use crate::{
    cart::value_objects::customer_id::CustomerId,
    order::{
        shop_order::{OrderItem, OrderState, ShopOrder},
        value_objects::shop_order_id::ShopOrderId,
    },
};

pub struct ShopOrderRestorer {}

impl ShopOrderRestorer {
    pub fn restore_order(
        id: ShopOrderId,
        created: OffsetDateTime,
        for_customer: CustomerId,
        address: Address,
        order_items: HashSet<OrderItem>,
        state: OrderState,
        version: Version,
    ) -> ShopOrder {
        ShopOrder {
            entity_params: DomainEntity::new(id, version),
            created,
            for_customer,
            address,
            order_items,
            state,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_fixtures::{rnd_address, rnd_customer_id, rnd_order_id, rnd_order_item};

    #[test]
    fn restore_user_success() {
        let id = rnd_order_id();
        let created = OffsetDateTime::now_utc();
        let customer_id = rnd_customer_id();
        let item = rnd_order_item();
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

        assert_eq!(order.id(), &id);
        assert_eq!(order.created(), &created);
        assert_eq!(order.for_customer(), &customer_id);
        assert_eq!(order.address(), &address);
        assert_eq!(order.order_items().len(), 1);
        let order_item = order.order_items().iter().next().unwrap().clone();
        assert_eq!(order_item.price, item.price);
        assert_eq!(order_item.meal_id, item.meal_id);
        assert_eq!(order_item.count, item.count);

        assert_eq!(order.state(), &state);
        assert_eq!(order.version(), &version);
        assert!(order.pop_events().is_empty());
    }
}
