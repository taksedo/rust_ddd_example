use std::collections::{HashMap, HashSet};

use bigdecimal::{BigDecimal, FromPrimitive};
use common::types::main::base::domain_entity::{DomainEntity, Version};
use common::types::main::base::domain_event::DomainEventTrait;
use common::types::main::common::address::Address;
use common::types::main::common::count::Count;
use derive_new::new;
use fake::faker::address::en::StreetName;
use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;
use rand::thread_rng;
use rand::Rng;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::main::cart::cart::Cart;
use crate::main::cart::cart_restorer::CartRestorer;
use crate::main::cart::value_objects::cart_id::CartId;
use crate::main::cart::value_objects::customer_id::CustomerId;
use crate::main::menu::meal::Meal;
use crate::main::menu::meal_already_exists::MealAlreadyExists;
use crate::main::menu::meal_restorer::MealRestorer;
use crate::main::menu::value_objects::meal_description::MealDescription;
use crate::main::menu::value_objects::meal_id::MealId;
use crate::main::menu::value_objects::meal_name::MealName;
use crate::main::menu::value_objects::price::Price;
use crate::main::order::shop_order::{OrderItem, OrderState, ShopOrder};
use crate::main::order::shop_order_id::ShopOrderId;

pub fn rnd_address() -> Address {
    Address::try_from((
        &*StreetName().fake::<String>(),
        thread_rng().gen_range(0..i16::MAX),
    ))
    .expect("Address should be right")
}

pub fn print_type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}

pub fn rnd_meal_id() -> MealId {
    let id: i64 = thread_rng().gen_range(0..i64::MAX);
    MealId::try_from(id).unwrap()
}

pub fn rnd_meal_name() -> MealName {
    MealName::try_from(Name(EN).fake::<String>().as_str()).unwrap()
}

pub fn rnd_meal_description() -> MealDescription {
    MealDescription::try_from(Name(EN).fake::<String>().as_str()).unwrap()
}

pub fn rnd_price() -> Price {
    let random_price: u64 = thread_rng().gen_range(0..500000);
    let price = Price::try_from(BigDecimal::from_u64(random_price).unwrap()).unwrap();
    Price::try_from(price.to_bigdecimal().with_scale(Price::SCALE)).unwrap()
}

pub fn version() -> Version {
    Version::default()
}

pub fn rnd_meal() -> Meal {
    MealRestorer::restore_meal(
        rnd_meal_id(),
        rnd_meal_name(),
        rnd_meal_description(),
        rnd_price(),
        false,
        Version::default(),
        vec![],
    )
}

#[derive(new, Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub struct TestEvent {}

impl DomainEventTrait for TestEvent {}

pub fn rnd_customer_id() -> CustomerId {
    CustomerId::new()
}

pub fn rnd_cart_id() -> CartId {
    CartId::new(thread_rng().gen_range(0..i64::MAX))
}

pub fn rnd_cart() -> Cart {
    CartRestorer::restore_cart(
        rnd_cart_id(),
        rnd_customer_id(),
        OffsetDateTime::now_utc(),
        HashMap::new(),
        version(),
    )
}

pub fn rnd_order_id() -> ShopOrderId {
    ShopOrderId::new(thread_rng().gen_range(0..i64::MAX))
}

pub fn rnd_order_item(price: Price, count: Count) -> OrderItem {
    OrderItem::new(rnd_meal_id(), price, count)
}

pub fn rnd_order(order_items: HashSet<OrderItem>) -> ShopOrder {
    ShopOrder::new(
        DomainEntity::new(rnd_order_id(), Default::default()),
        OffsetDateTime::now_utc(),
        rnd_customer_id(),
        rnd_address(),
        order_items,
        OrderState::new_completed(),
    )
}

pub fn order_with_state(state: OrderState) -> ShopOrder {
    ShopOrder::new(
        DomainEntity::new(rnd_order_id(), Default::default()),
        OffsetDateTime::now_utc(),
        rnd_customer_id(),
        rnd_address(),
        HashSet::new(),
        state,
    )
}

#[derive(Debug, new, Default, Clone, Copy)]
pub struct TestMealAlreadyExists {
    #[new(value = "false")]
    pub value: bool,
}

impl MealAlreadyExists for TestMealAlreadyExists {
    fn invoke(&mut self, _name: &MealName) -> bool {
        self.value
    }
}
