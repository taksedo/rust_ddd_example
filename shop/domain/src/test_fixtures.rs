use std::collections::{HashMap, HashSet};

use async_trait::async_trait;
use bigdecimal::{BigDecimal, FromPrimitive};
use common::{
    test_fixtures::rnd_count,
    types::{
        base::{DomainEntity, DomainEventTrait, Version},
        common::{Address, Count},
    },
};
use derive_new::new;
use fake::{
    Fake,
    faker::{address::en::StreetName, name::raw::*},
    locales::*,
};
use rand::random_range;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{
    cart::{
        cart::Cart,
        cart_restorer::CartRestorer,
        value_objects::{cart_id::CartId, customer_id::CustomerId},
    },
    menu::{
        meal::Meal,
        meal_already_exists::MealAlreadyExists,
        meal_restorer::MealRestorer,
        value_objects::{
            meal_description::MealDescription, meal_id::MealId, meal_name::MealName, price::Price,
        },
    },
    order::{
        shop_order::{OrderItem, OrderState, ShopOrder},
        value_objects::shop_order_id::ShopOrderId,
    },
};

pub fn rnd_address() -> Address {
    Address::try_from((&*StreetName().fake::<String>(), random_range(0..i16::MAX)))
        .expect("Address should be right")
}

pub fn rnd_meal_id() -> MealId {
    let id: i64 = random_range(0..i64::MAX);
    MealId::try_from(id).unwrap()
}

pub fn rnd_meal_name() -> MealName {
    MealName::try_from(Name(EN).fake::<String>().as_str()).unwrap()
}

pub fn rnd_meal_description() -> MealDescription {
    let val = Name(EN).fake::<String>();
    MealDescription::try_from(val.as_str()).unwrap()
}

pub fn rnd_price() -> Price {
    let random_price: u64 = random_range(0..500000);
    let price = Price::try_from(BigDecimal::from_u64(random_price).unwrap()).unwrap();
    Price::try_from(price.to_bigdecimal().with_scale(Price::SCALE)).unwrap()
}

pub fn version() -> Version {
    Version::default()
}

pub fn rnd_meal() -> Meal {
    MealRestorer::restore_meal(
        &rnd_meal_id(),
        &rnd_meal_name(),
        &rnd_meal_description(),
        &rnd_price(),
        false,
        &Version::default(),
        vec![],
    )
}

pub fn rnd_removed_meal() -> Meal {
    MealRestorer::restore_meal(
        &rnd_meal_id(),
        &rnd_meal_name(),
        &rnd_meal_description(),
        &rnd_price(),
        true,
        &Version::default(),
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
    CartId::try_from(random_range(0..i64::MAX)).unwrap()
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

pub fn rnd_cart_with_customer_id(customer_id: CustomerId) -> Cart {
    CartRestorer::restore_cart(
        rnd_cart_id(),
        customer_id,
        OffsetDateTime::now_utc(),
        HashMap::new(),
        version(),
    )
}

pub fn rnd_cart_with_customer_id_and_meals(
    customer_id: CustomerId,
    meals: HashMap<MealId, Count>,
) -> Cart {
    CartRestorer::restore_cart(
        rnd_cart_id(),
        customer_id,
        OffsetDateTime::now_utc(),
        meals,
        version(),
    )
}

pub fn rnd_order_id() -> ShopOrderId {
    ShopOrderId::try_from(random_range(0..i64::MAX)).unwrap()
}

pub fn rnd_order_item() -> OrderItem {
    OrderItem::new(rnd_meal_id(), rnd_price(), rnd_count())
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

pub fn rnd_order_with_customer_id(customer_id: CustomerId) -> ShopOrder {
    ShopOrder::new(
        DomainEntity::new(rnd_order_id(), Default::default()),
        OffsetDateTime::now_utc(),
        customer_id,
        rnd_address(),
        [rnd_order_item()].into(),
        OrderState::new_completed(),
    )
}

pub fn rnd_order_with_id(id: ShopOrderId) -> ShopOrder {
    ShopOrder::new(
        DomainEntity::new(id, Default::default()),
        OffsetDateTime::now_utc(),
        rnd_customer_id(),
        rnd_address(),
        [rnd_order_item()].into(),
        OrderState::new_completed(),
    )
}

pub fn order_with_state(state: OrderState) -> ShopOrder {
    ShopOrder::new(
        DomainEntity::new(rnd_order_id(), Default::default()),
        OffsetDateTime::now_utc(),
        rnd_customer_id(),
        rnd_address(),
        HashSet::from([rnd_order_item()]),
        state,
    )
}

#[derive(Debug, new, Default, Clone, Copy)]
pub struct TestMealAlreadyExists {
    #[new(value = "false")]
    pub value: bool,
}

#[async_trait]
impl MealAlreadyExists for TestMealAlreadyExists {
    async fn invoke(&mut self, _name: &MealName) -> bool {
        self.value
    }
}
