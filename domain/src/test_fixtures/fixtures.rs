use crate::main::menu::meal_id::MealId;
use crate::main::menu::meal_name::MealName;
use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;
use rand::thread_rng;
use rand::Rng;

//
// fn address() = Address.from(
// street = faker.address().streetName(),
// building = faker.address().streetAddressNumber().toInt() + 1
// ).getOrElse { error("Address should be right") }

pub fn rnd_meal_name() -> MealName {
    MealName::from(Name(EN).fake()).unwrap()
}

pub fn rnd_meal_id() -> MealId {
    let id: i64 = thread_rng().gen_range(0..i64::MAX);
    MealId { value: id }
}

// pub fn rnd_meal(id: MealId, removed: bool) -> Meal {
//     MealRestorer::restore_meal(id, rnd_meal_name())
// }
//
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
