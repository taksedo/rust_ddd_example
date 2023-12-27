use std::collections::HashSet;

use common::types::main::{
    base::domain_entity::{DomainEntity, Version},
    common::address::Address,
};
use time::OffsetDateTime;

use crate::main::{
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
