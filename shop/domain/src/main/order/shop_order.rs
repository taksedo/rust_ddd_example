use std::{
    collections::{hash_map::DefaultHasher, HashSet},
    hash::{Hash, Hasher},
    sync::{Arc, Mutex},
};

use common::types::{
    base::domain_entity::{DomainEntity, DomainEntityTrait},
    common::{address::Address, count::Count},
};
use derive_new::new;
use serde_derive::{Deserialize, Serialize};
use smart_default::SmartDefault;
use time::OffsetDateTime;

use crate::main::{
    cart::{cart::Cart, value_objects::customer_id::CustomerId},
    menu::value_objects::{meal_id::MealId, price::Price},
    order::{
        customer_has_active_order::CustomerHasActiveOrder,
        customer_order_events::{
            ShopOrderCancelledDomainEvent, ShopOrderCompletedDomainEvent,
            ShopOrderConfirmedDomainEvent, ShopOrderCreatedDomainEvent, ShopOrderEventEnum,
            ShopOrderPaidDomainEvent,
        },
        get_meal_price::GetMealPrice,
        shop_order::OrderState::{Cancelled, Completed, Confirmed, Paid, WaitingForPayment},
        value_objects::shop_order_id::{ShopOrderId, ShopOrderIdGenerator},
    },
};

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

    pub fn confirm(&mut self) -> Result<(), InvalidState> {
        self.change_state(
            OrderState::new_confirmed(),
            ShopOrderConfirmedDomainEvent::new(self.entity_params.id).into(),
        )
    }

    pub fn pay(&mut self) -> Result<(), InvalidState> {
        self.change_state(
            OrderState::new_paid(),
            ShopOrderPaidDomainEvent::new(self.entity_params.id).into(),
        )
    }

    pub fn complete(&mut self) -> Result<(), InvalidState> {
        self.change_state(
            OrderState::new_completed(),
            ShopOrderCompletedDomainEvent::new(self.entity_params.id).into(),
        )
    }

    pub fn cancel(&mut self) -> Result<(), InvalidState> {
        self.change_state(
            OrderState::new_cancelled(),
            ShopOrderCancelledDomainEvent::new(self.entity_params.id).into(),
        )
    }

    pub fn change_state(
        &mut self,
        new_state: OrderState,
        event: ShopOrderEventEnum,
    ) -> Result<(), InvalidState> {
        if self.state == new_state {
            Ok(())
        } else if self.state.can_change_to(&new_state) {
            self.state = new_state;
            self.entity_params.add_event(event);
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

#[derive(new, PartialEq, Eq, Debug, Clone, Hash, SmartDefault, Serialize, Deserialize)]
pub enum OrderState {
    Cancelled(#[new(value = "false")] bool),
    Completed(#[new(value = "false")] bool),
    Confirmed(#[new(value = "true")] bool),
    Paid(#[new(value = "true")] bool),
    #[default]
    WaitingForPayment(#[new(value = "true")] bool),
}

impl OrderState {
    pub fn can_change_to(&self, state: &OrderState) -> bool {
        match self {
            Confirmed(_) => matches!(state, Completed(_)),

            Paid(_) => {
                matches!(state, Confirmed(_) | Cancelled(_))
            }

            WaitingForPayment(_) => matches!(state, Paid(_)),

            _ => false,
        }
    }

    pub fn is_active(&self) -> bool {
        match &self {
            Cancelled(value) => *value,
            Completed(value) => *value,
            Confirmed(value) => *value,
            Paid(value) => *value,
            WaitingForPayment(value) => *value,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CheckoutError {
    EmptyCart,
    AlreadyHasActiveOrder,
}

#[derive(Debug, PartialEq)]
pub struct InvalidState;

#[derive(Debug, PartialEq)]
pub enum ShopOrderError {
    IdGenerationError,
}
