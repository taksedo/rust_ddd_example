use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use common::types::main::base::domain_entity::DomainEntityTrait;
use common::types::test_fixtures::rnd_count;
use derive_new::new;
use smart_default::SmartDefault;

use crate::main::cart::value_objects::customer_id::CustomerId;
use crate::main::menu::value_objects::meal_id::MealId;
use crate::main::menu::value_objects::price::Price;
use crate::main::order::customer_has_active_order::CustomerHasActiveOrder;
use crate::main::order::customer_order_events::{
    ShopOrderCreatedDomainEvent, ShopOrderPaidDomainEvent,
};
use crate::main::order::get_meal_price::GetMealPrice;
use crate::main::order::shop_order::{CheckoutError, OrderItem, OrderState, ShopOrder};
use crate::main::order::shop_order_id::{ShopOrderId, ShopOrderIdGenerator};
use crate::test_fixtures::{
    order_with_state, rnd_address, rnd_cart, rnd_meal_id, rnd_order_id, rnd_price,
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
fn complete_order_success() {
    let mut order = order_with_state(OrderState::new_waiting_for_payment());
    assert_eq!(order.pay(), ());
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
    assert_eq!(order.complete(), ());
    assert!(matches!(order.state, OrderState::Completed(_)));
    assert!(order.entity_params.pop_events().is_empty());
}

#[derive(new, Default)]
struct HashMapStoragePriceProvider {
    storage: HashMap<MealId, Price>,
}

impl GetMealPrice for HashMapStoragePriceProvider {
    fn invoke(&self, for_meal_id: MealId) -> Price {
        let result = &self.storage.get(&for_meal_id);
        result.unwrap().clone()
    }
}

#[derive(SmartDefault)]
struct MockOrderIdGenerator {
    #[default(rnd_order_id())]
    id: ShopOrderId,
}

impl ShopOrderIdGenerator for MockOrderIdGenerator {
    fn generate(&self) -> ShopOrderId {
        self.id
    }
}

#[derive(new)]
struct MockCustomerHasActiveOrder {
    status: bool,
}

impl CustomerHasActiveOrder for MockCustomerHasActiveOrder {
    fn invoke(&self, _for_customer: CustomerId) -> bool {
        self.status
    }
}
