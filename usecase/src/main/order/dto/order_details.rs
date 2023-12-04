use common::types::main::base::domain_entity::Version;
use common::types::main::common::address::Address;
use common::types::main::common::count::Count;
use derive_new::new;

use domain::main::menu::value_objects::meal_id::MealId;
use domain::main::menu::value_objects::price::Price;
use domain::main::order::shop_order::{OrderState, ShopOrder};
use domain::main::order::value_objects::shop_order_id::ShopOrderId;

pub struct OrderDetails {
    pub id: ShopOrderId,
    pub state: OrderState,
    pub address: Address,
    pub ready_for_confirm_or_cancel: bool,
    pub items: Vec<OrderItemDetails>,
    pub total: Price,
    pub version: Version,
}

pub trait ToDetails {
    fn to_details(&self) -> OrderDetails;
}

impl ToDetails for ShopOrder {
    fn to_details(&self) -> OrderDetails {
        let items: Vec<OrderItemDetails> = self
            .order_items
            .iter()
            .map(|it| OrderItemDetails::new(it.meal_id, it.count))
            .collect();
        OrderDetails {
            id: self.entity_params.id,
            state: self.state.clone(),
            address: self.address.clone(),
            ready_for_confirm_or_cancel: self.ready_for_confirm_or_cancel(),
            items,
            total: self.total_price(),
            version: self.entity_params.version,
        }
    }
}

#[derive(new, Copy, Clone, Debug, PartialEq)]
pub struct OrderItemDetails {
    pub meal_id: MealId,
    pub count: Count,
}
