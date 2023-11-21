use bigdecimal::{BigDecimal, FromPrimitive};
use derive_new::new;
use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;
use rand::thread_rng;
use rand::Rng;
use serde::{Deserialize, Serialize};

use common_types::main::base::domain_entity::Version;
use common_types::main::base::domain_event::DomainEventTrait;

use crate::main::menu::meal::Meal;
use crate::main::menu::meal_already_exists::MealAlreadyExists;
use crate::main::menu::meal_restorer::MealRestorer;
use crate::main::menu::value_objects::meal_description::MealDescription;
use crate::main::menu::value_objects::meal_id::MealId;
use crate::main::menu::value_objects::meal_name::MealName;
use crate::main::menu::value_objects::price::Price;

//
// fn address() = Address.from(
// street = faker.address().streetName(),
// building = faker.address().streetAddressNumber().toInt() + 1
// ).getOrElse { error("Address should be right") }

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
    Price::try_from(price.value.with_scale(2)).unwrap()
}

pub fn version() -> Version {
    Version::new()
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

// fn customerId() = CustomerId(UUID.randomUUID().toString())
//
// fn cartId() = CartId(faker.number().randomNumber())
//
// fn cart(
// meals: Map<MealId, Count> = emptyMap(),
// customerId: CustomerId = customerId(),
// ): Cart {
// return CartRestorer.restoreCart(
// id = cartId(),
// forCustomer = customerId,
// created = OffsetDateTime.now(),
// meals = meals,
// version = version()
// )
// }
//
// fn orderId() = ShopOrderId(faker.number().randomNumber())
//
// fn orderItem(
// price: Price = price(),
// count: Count = count(),
// ): OrderItem {
// return OrderItem(
// meal_id = meal_id(),
// price = price,
// count = count
// )
// }
//
// fn order(
// id: ShopOrderId = orderId(),
// customerId: CustomerId = customerId(),
// state: OrderState = OrderState.COMPLETED,
// orderItems: Set<OrderItem> = setOf(orderItem()),
// ): ShopOrder {
// return ShopOrderRestorer.restoreOrder(
// id = id,
// created = OffsetDateTime.now(),
// forCustomer = customerId,
// orderItems = orderItems,
// address = address(),
// state = state,
// version = version()
// )
// }

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
