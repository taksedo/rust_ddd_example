use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use bigdecimal::num_bigint::BigInt;
use bigdecimal::BigDecimal;
use common::types::main::base::domain_entity::DomainEntityTrait;
use common::types::main::common::count::Count;
use common::types::test_fixtures::rnd_count;
use derive_new::new;
use smart_default::SmartDefault;

use crate::main::cart::value_objects::customer_id::CustomerId;
use crate::main::menu::value_objects::meal_id::MealId;
use crate::main::menu::value_objects::price::Price;
use crate::main::order::customer_has_active_order::CustomerHasActiveOrder;
use crate::main::order::customer_order_events::{
    ShopOrderCancelledDomainEvent, ShopOrderConfirmedDomainEvent, ShopOrderCreatedDomainEvent,
    ShopOrderPaidDomainEvent,
};
use crate::main::order::get_meal_price::GetMealPrice;
use crate::main::order::shop_order::{
    CheckoutError, InvalidState, OrderItem, OrderState, ShopOrder,
};
use crate::main::order::value_objects::shop_order_id::{ShopOrderId, ShopOrderIdGenerator};
use crate::test_fixtures::{
    order_with_state, rnd_address, rnd_cart, rnd_meal_id, rnd_order, rnd_order_id, rnd_order_item,
    rnd_price,
};

#[test]
fn checkout_success() {
    let id_generator = Arc::new(Mutex::new(MockOrderIdGenerator::default()));
    let id = id_generator.lock().unwrap().id;
    let meal_id = rnd_meal_id();
    let count = rnd_count();
    let price = rnd_price();
    let address = rnd_address();

    let get_meal_price = Arc::new(Mutex::new(HashMapStoragePriceProvider::default()));
    get_meal_price
        .lock()
        .unwrap()
        .storage
        .insert(meal_id, price.clone());
    let mut cart = rnd_cart();
    cart.meals.insert(meal_id, count);

    let result = ShopOrder::checkout(
        cart.clone(),
        Arc::clone(&id_generator) as _,
        Arc::new(Mutex::new(MockCustomerHasActiveOrder::new(false))) as _,
        address.clone(),
        Arc::clone(&get_meal_price) as _,
    );

    let mut order = result.unwrap();

    assert_eq!(order.for_customer, cart.for_customer);
    assert_eq!(
        order.order_items,
        HashSet::from([OrderItem::new(meal_id, price, count)])
    );
    assert_eq!(order.entity_params.id, id);
    assert_eq!(order.address, address);
    assert!(matches!(order.state, OrderState::WaitingForPayment(_)));
    let events: Vec<ShopOrderCreatedDomainEvent> = order
        .entity_params
        .pop_events()
        .iter()
        .map(|it| it.clone().try_into().unwrap())
        .collect();
    assert_eq!(events.len(), 1);
    let event = events.first().unwrap().clone();
    assert_eq!(event.order_id, id);
    assert_eq!(event.for_customer, cart.for_customer);
    assert_eq!(event.total_price, order.total_price());
}

#[test]
fn checkout_already_has_active_user() {
    let id_generator = Arc::new(Mutex::new(MockOrderIdGenerator::default()));
    let meal_id = rnd_meal_id();
    let count = rnd_count();
    let price = rnd_price();
    let address = rnd_address();

    let meal_price_only_for_special_meal =
        Arc::new(Mutex::new(HashMapStoragePriceProvider::default()));
    meal_price_only_for_special_meal
        .lock()
        .unwrap()
        .storage
        .insert(meal_id, price);

    let mut cart = rnd_cart();
    cart.meals.insert(meal_id, count);

    let result = ShopOrder::checkout(
        cart.clone(),
        Arc::clone(&id_generator) as _,
        Arc::new(Mutex::new(MockCustomerHasActiveOrder::new(true))) as _,
        address.clone(),
        Arc::clone(&meal_price_only_for_special_meal) as _,
    );

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), CheckoutError::AlreadyHasActiveOrder);
}

#[test]
fn checkout_empty_cart() {
    let id_generator = Arc::new(Mutex::new(MockOrderIdGenerator::default()));
    let cart = rnd_cart();
    let get_meal_price = Arc::new(Mutex::new(HashMapStoragePriceProvider::default()));
    get_meal_price
        .lock()
        .unwrap()
        .storage
        .insert(rnd_meal_id(), rnd_price());
    let result = ShopOrder::checkout(
        cart.clone(),
        Arc::clone(&id_generator) as _,
        Arc::new(Mutex::new(MockCustomerHasActiveOrder::new(false))) as _,
        rnd_address(),
        Arc::clone(&get_meal_price) as _,
    );

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), CheckoutError::EmptyCart)
}

#[test]
fn active_true() {
    let states = vec![
        OrderState::new_waiting_for_payment(),
        OrderState::new_confirmed(),
        OrderState::new_paid(),
    ];

    states.iter().for_each(|it| {
        dbg!(&it);
        assert!(it.is_active())
    });
}

#[test]
fn active_false() {
    let states = vec![OrderState::new_completed(), OrderState::new_cancelled()];

    states.iter().for_each(|it| assert!(!it.is_active()));
}
#[test]
fn complete_order_success() {
    let mut order = order_with_state(OrderState::new_waiting_for_payment());
    assert!(order.pay().is_ok());
    assert!(matches!(order.state, OrderState::Paid(_)));
    let event: Vec<ShopOrderPaidDomainEvent> = order
        .entity_params
        .pop_events()
        .iter()
        .map(|it| it.clone().try_into().unwrap())
        .collect();
    assert_eq!(event.len(), 1);
    assert_eq!(event.first().unwrap().order_id, order.entity_params.id);
}

#[test]
fn complete_order_already() {
    let mut order = order_with_state(OrderState::new_completed());
    assert!(order.complete().is_ok());
    assert!(matches!(order.state, OrderState::Completed(_)));
    assert!(order.entity_params.pop_events().is_empty());
}

#[test]
fn complete_order_invalid_state() {
    let states = vec![
        OrderState::new_waiting_for_payment(),
        OrderState::new_paid(),
        OrderState::new_cancelled(),
    ];

    states.iter().for_each(|state| {
        let mut order = order_with_state(state.clone());
        assert_eq!(order.complete().unwrap_err(), InvalidState);
        assert_eq!(order.state, state.clone());
        assert!(order.entity_params.pop_events().is_empty())
    });
}

#[test]
fn pay_order_success() {
    let mut order = order_with_state(OrderState::new_waiting_for_payment());
    assert!(order.pay().is_ok());
    assert!(matches!(order.state, OrderState::Paid(_)));
    let event: Vec<ShopOrderPaidDomainEvent> = order
        .entity_params
        .pop_events()
        .iter()
        .map(|it| it.clone().try_into().unwrap())
        .collect();
    assert_eq!(event.len(), 1);
    assert_eq!(event.first().unwrap().order_id, order.entity_params.id);
}

#[test]
fn pay_order_already() {
    let mut order = order_with_state(OrderState::new_paid());
    assert!(order.pay().is_ok());
    assert!(matches!(order.state, OrderState::Paid(_)));
    assert!(order.entity_params.pop_events().is_empty());
}

#[test]
fn pay_order_invalid_state() {
    let states = vec![
        OrderState::new_confirmed(),
        OrderState::new_completed(),
        OrderState::new_cancelled(),
    ];

    states.iter().for_each(|state| {
        let mut order = order_with_state(state.clone());
        assert_eq!(order.pay().unwrap_err(), InvalidState);
        assert_eq!(order.state, state.clone());
        assert!(order.entity_params.pop_events().is_empty())
    });
}

#[test]
fn order_is_ready_to_confirm_or_cancel() {
    let order = order_with_state(OrderState::new_paid());
    assert!(order.ready_for_confirm_or_cancel());
}

#[test]
fn order_cannot_be_cancelled() {
    let states = vec![
        OrderState::new_confirmed(),
        OrderState::new_completed(),
        OrderState::new_waiting_for_payment(),
        OrderState::new_cancelled(),
    ];

    states.iter().for_each(|state| {
        let order = order_with_state(state.clone());
        assert!(!order.ready_for_confirm_or_cancel());
    });
}

#[test]
fn cancel_order_success() {
    let mut order = order_with_state(OrderState::new_paid());
    assert!(order.cancel().is_ok());
    assert!(matches!(order.state, OrderState::Cancelled(_)));
    let event: Vec<ShopOrderCancelledDomainEvent> = order
        .entity_params
        .pop_events()
        .iter()
        .map(|it| it.clone().try_into().unwrap())
        .collect();
    assert_eq!(event.len(), 1);
    assert_eq!(event.first().unwrap().order_id, order.entity_params.id);
}

#[test]
fn cancel_order_already() {
    let mut order = order_with_state(OrderState::new_cancelled());
    assert!(order.cancel().is_ok());
    assert!(matches!(order.state, OrderState::Cancelled(_)));
    assert!(order.entity_params.pop_events().is_empty());
}

#[test]
fn cancel_order_invalid_state() {
    let states = vec![
        OrderState::new_confirmed(),
        OrderState::new_completed(),
        OrderState::new_waiting_for_payment(),
    ];

    states.iter().for_each(|state| {
        let mut order = order_with_state(state.clone());
        assert_eq!(order.cancel().unwrap_err(), InvalidState);
        assert_eq!(order.state, state.clone());
        assert!(order.entity_params.pop_events().is_empty())
    });
}

#[test]
fn confirm_order_success() {
    let mut order = order_with_state(OrderState::new_paid());
    assert!(order.confirm().is_ok());
    assert!(matches!(order.state, OrderState::Confirmed(_)));
    let event: Vec<ShopOrderConfirmedDomainEvent> = order
        .entity_params
        .pop_events()
        .iter()
        .map(|it| it.clone().try_into().unwrap())
        .collect();
    assert_eq!(event.len(), 1);
    assert_eq!(event.first().unwrap().order_id, order.entity_params.id);
}

#[test]
fn confirm_order_already() {
    let mut order = order_with_state(OrderState::new_confirmed());
    assert!(order.confirm().is_ok());
    assert!(matches!(order.state, OrderState::Confirmed(_)));
    assert!(order.entity_params.pop_events().is_empty());
}

#[test]
fn confirm_order_invalid_state() {
    let states = vec![
        OrderState::new_cancelled(),
        OrderState::new_completed(),
        OrderState::new_waiting_for_payment(),
    ];

    states.iter().for_each(|state| {
        let mut order = order_with_state(state.clone());
        assert_eq!(order.confirm().unwrap_err(), InvalidState);
        assert_eq!(order.state, state.clone());
        assert!(order.entity_params.pop_events().is_empty())
    });
}

#[test]
fn calculate_total() {
    let order_item_1 = rnd_order_item(
        Price::try_from(BigDecimal::from_str("1.03").unwrap()).unwrap(),
        Count::try_from(2).unwrap(),
    );
    let order_item_2 = rnd_order_item(
        Price::try_from(BigDecimal::from_str("91.33").unwrap()).unwrap(),
        Count::try_from(4).unwrap(),
    );

    let order = rnd_order(HashSet::from([order_item_1, order_item_2]));
    assert_eq!(
        order.total_price(),
        Price::try_from(BigDecimal::new(BigInt::from(36738), 2)).unwrap()
    )
}

#[derive(new, Default, Debug)]
struct HashMapStoragePriceProvider {
    storage: HashMap<MealId, Price>,
}

impl GetMealPrice for HashMapStoragePriceProvider {
    fn invoke(&self, for_meal_id: MealId) -> Price {
        let result = &self.storage.get(&for_meal_id);
        result.unwrap().clone()
    }
}

#[derive(SmartDefault, Debug)]
struct MockOrderIdGenerator {
    #[default(rnd_order_id())]
    id: ShopOrderId,
}

impl ShopOrderIdGenerator for MockOrderIdGenerator {
    fn generate(&mut self) -> ShopOrderId {
        self.id
    }
}

#[derive(new, Debug)]
struct MockCustomerHasActiveOrder {
    status: bool,
}

impl CustomerHasActiveOrder for MockCustomerHasActiveOrder {
    fn invoke(&mut self, _for_customer: CustomerId) -> bool {
        self.status
    }
}
