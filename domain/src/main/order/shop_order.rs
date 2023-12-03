use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

use common::types::main::base::domain_entity::{DomainEntity, DomainEntityTrait};
use common::types::main::common::address::Address;
use common::types::main::common::count::Count;
use derive_new::new;
use serde_derive::{Deserialize, Serialize};
use smart_default::SmartDefault;
use time::OffsetDateTime;

use crate::main::cart::cart::Cart;
use crate::main::cart::value_objects::customer_id::CustomerId;
use crate::main::menu::value_objects::meal_id::MealId;
use crate::main::menu::value_objects::price::Price;
use crate::main::order::customer_has_active_order::CustomerHasActiveOrder;
use crate::main::order::customer_order_events::{
    ShopOrderCancelledDomainEvent, ShopOrderCompletedDomainEvent, ShopOrderConfirmedDomainEvent,
    ShopOrderCreatedDomainEvent, ShopOrderEventEnum, ShopOrderPaidDomainEvent,
};
use crate::main::order::get_meal_price::GetMealPrice;
use crate::main::order::shop_order::OrderState::{
    Cancelled, Completed, Confirmed, Paid, WaitingForPayment,
};
use crate::main::order::shop_order_id::{ShopOrderId, ShopOrderIdGenerator};

#[derive(new, Debug, Clone, PartialEq, Serialize, Deserialize, SmartDefault)]
pub struct ShopOrder {
    pub entity_params: DomainEntity<ShopOrderId, ShopOrderEventEnum>,
    #[default(OffsetDateTime::now_utc())]
    pub created: OffsetDateTime,
    pub for_customer: CustomerId,
    pub address: Address,
    pub order_items: HashSet<OrderItem>,
    pub state: OrderState,
}

impl ShopOrder {
    pub fn checkout(
        cart: Cart,
        id_generator: Arc<Mutex<dyn ShopOrderIdGenerator>>,
        customer_has_active_order: Arc<Mutex<dyn CustomerHasActiveOrder>>,
        address: Address,
        get_meal_price: Arc<Mutex<dyn GetMealPrice>>,
    ) -> Result<ShopOrder, CheckoutError> {
        if customer_has_active_order
            .lock()
            .unwrap()
            .invoke(cart.for_customer)
        {
            return Err(CheckoutError::AlreadyHasActiveOrder);
        }
        let meals = cart.meals;
        if !meals.is_empty() {
            let mut set = HashSet::new();

            for (meal_id, count) in meals.iter() {
                let price = get_meal_price.lock().unwrap().invoke(*meal_id);
                set.insert(OrderItem::new(*meal_id, price, *count));
            }
            let id = id_generator.lock().unwrap().generate();
            let mut shop_order = ShopOrder::new(
                DomainEntity::new(id, Default::default()),
                OffsetDateTime::now_utc(),
                cart.for_customer,
                address,
                set,
                Default::default(),
            );
            let total_price = shop_order.total_price();
            shop_order.entity_params.add_event(
                ShopOrderCreatedDomainEvent::new(id, cart.for_customer, total_price).into(),
            );
            Ok(shop_order)
        } else {
            Err(CheckoutError::EmptyCart)
        }
    }

    pub fn confirm(&mut self) {
        self.change_state(
            Confirmed(true),
            ShopOrderConfirmedDomainEvent::new(self.entity_params.id).into(),
        )
        .expect("TODO: panic message");
    }

    pub fn pay(&mut self) {
        self.change_state(
            Paid(true),
            ShopOrderPaidDomainEvent::new(self.entity_params.id).into(),
        )
        .expect("TODO: panic message");
    }

    pub fn complete(&mut self) {
        self.change_state(
            Completed(true),
            ShopOrderCompletedDomainEvent::new(self.entity_params.id).into(),
        )
        .expect("TODO: panic message");
    }

    pub fn cancel(&mut self) {
        self.change_state(
            Cancelled(true),
            ShopOrderCancelledDomainEvent::new(self.entity_params.id).into(),
        )
        .expect("TODO: panic message");
    }

    pub fn change_state(
        &mut self,
        new_state: OrderState,
        _event: ShopOrderEventEnum,
    ) -> Result<(), InvalidState> {
        if self.state == new_state {
            Ok(())
        } else if self.state.can_change_to(&new_state) {
            self.state = new_state;
            Ok(())
        } else {
            Err(InvalidState)
        }
    }
    pub fn total_price(&self) -> Price {
        self.order_items
            .iter()
            .map(|it| it.price.multiple(it.count))
            .fold(Price::zero(), |acc, it| acc.add(it))
    }

    pub fn is_active(&self) -> bool {
        match &self.state {
            Cancelled(value) => *value,
            Completed(value) => *value,
            Confirmed(value) => *value,
            Paid(value) => *value,
            WaitingForPayment(value) => *value,
        }
    }

    pub fn ready_for_confirm_or_cancel(&self) -> bool {
        matches!(&self.state, Paid(_))
    }
}

#[derive(new, Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub struct OrderItem {
    pub meal_id: MealId,
    pub price: Price,
    pub count: Count,
}

impl OrderItem {
    pub fn hash_code(&self) -> u64 {
        let mut state = DefaultHasher::new();
        self.meal_id.hash(&mut state);
        state.finish()
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash, SmartDefault, Serialize, Deserialize)]
pub enum OrderState {
    Cancelled(bool),
    Completed(bool),
    Confirmed(bool),
    Paid(bool),
    #[default]
    WaitingForPayment(bool),
}

impl OrderState {
    pub fn new(active: bool) -> Self {
        match active {
            false => Cancelled(false),
            true => Confirmed(true),
        }
    }

    pub fn can_change_to(&self, state: &OrderState) -> bool {
        match self {
            Confirmed(_) => matches!(state, Completed(_)),

            Paid(_) => {
                matches!(state, Confirmed(_) | Cancelled(_))
            }

            OrderState::WaitingForPayment(_) => matches!(state, Paid(_)),

            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CheckoutError {
    EmptyCart,
    AlreadyHasActiveOrder,
}

#[derive(Debug)]
pub struct InvalidState;
