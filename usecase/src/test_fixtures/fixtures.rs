use crate::main::menu::access::meal_extractor::MealExtractor;
use crate::main::menu::access::meal_persister::MealPersister;
use common_types::main::base::domain_event::DomainEventTrait;
use derive_new::new;
use domain::main::menu::meal::Meal;
use domain::main::menu::meal_events::MealRemovedFromMenuDomainEvent;
use domain::main::menu::meal_id::MealId;
use domain::main::menu::meal_name::MealName;
use domain::test_fixtures::fixtures::rnd_meal;
use std::collections::HashMap;

pub fn removed_meal() -> Meal<MealRemovedFromMenuDomainEvent> {
    let mut meal = rnd_meal();
    meal.remove_meal_from_menu();
    meal
}

// fn orderReadyForPay() = order(state = OrderState.WAITING_FOR_PAYMENT)
//
// fn orderNotReadyForPay() = order(state = OrderState.COMPLETED)
//
// fn orderReadyForCancel() = order(state = OrderState.PAID)
//
// fn orderNotReadyForCancel() = order(state = OrderState.COMPLETED)
//
// fn orderReadyForConfirm() = order(state = OrderState.PAID)
//
// fn orderNotReadyForConfirm() = order(state = OrderState.WAITING_FOR_PAYMENT)
//
// fn orderReadyForComplete() = order(state = OrderState.CONFIRMED)
//
// fn orderNotReadyForComplete() = order(state = OrderState.CANCELLED)
//
// fn activeOrder() = order(state = OrderState.CONFIRMED)
//
// fn nonActiveOrder() = order(state = OrderState.CANCELLED)
//

#[derive(new, Debug, Clone)]
pub struct TestEvent {}

impl DomainEventTrait for TestEvent {}

#[derive(new, Debug, Clone)]
pub struct TestMealPersister<E: DomainEventTrait + Clone> {
    #[new(value = "HashMap::new()")]
    pub value: HashMap<MealId, Meal<E>>,
}

impl<E: DomainEventTrait + Clone> MealPersister<E> for TestMealPersister<E> {
    fn save(&mut self, meal: Meal<E>) {
        self.value.insert(meal.id, meal);
    }
}

//
// class MockCartPersister : CartPersister {
//
// lateinit var cart: Cart
//
// override fn save(cart: Cart) {
// this.cart = cart
// }
//
// fn verify_invoked(cart: Cart) {
// this.cart shouldBe cart
// }
//
// fn verify_invoked(cart: Cart, idMeal: MealId) {
// this.cart shouldBe cart
// this.cart.meals() shouldContainExactly mapOf(idMeal to count(1))
// }
//
// fn verify_invoked(id: CartId, customerId: CustomerId, idMeal: MealId) {
// this.cart.id shouldBe id
// this.cart.forCustomer shouldBe customerId
// this.cart.meals() shouldContainExactly mapOf(idMeal to count(1))
// }
//
// fn verify_empty() {
// ::cart.isInitialized shouldBe false
// }
// }
//
// class MockShopOrderPersister : ShopOrderPersister {
//
// lateinit var order: ShopOrder
//
// override fn save(order: ShopOrder) {
// this.order = order
// }
//
// fn verify_invoked(order: ShopOrder) {
// this.order shouldBe order
// }
//
// fn verify_invoked(
// orderId: ShopOrderId, address: Address, customerId: CustomerId,
// mealId: MealId, countItems: Count, priceItems: Price
// ) {
// this.order.id shouldBe orderId
// this.order.address shouldBe address
// this.order.forCustomer shouldBe customerId
// this.order.orderItems.shouldHaveSize(1)
//
// val orderItem = this.order.orderItems.first()
// orderItem.mealId shouldBe mealId
// orderItem.count shouldBe countItems
// orderItem.price shouldBe priceItems
// }
//
// fn verifyEventsAfterCancellation(id: ShopOrderId) {
// this.order.popEvents() shouldContainExactly listOf(ShopOrderCancelledDomainEvent(id))
// }
//
// fn verifyEventsAfterCompletion(id: ShopOrderId) {
// this.order.popEvents() shouldContainExactly listOf(ShopOrderCompletedDomainEvent(id))
// }
//
// fn verifyEventsAfterConfirmation(id: ShopOrderId) {
// this.order.popEvents() shouldContainExactly listOf(ShopOrderConfirmedDomainEvent(id))
// }
//
// fn verifyEventsAfterPayment(id: ShopOrderId) {
// this.order.popEvents() shouldContainExactly listOf(ShopOrderPaidDomainEvent(id))
// }
//
// fn verifyPrice(price: Price) {
// this.order.totalPrice() shouldBe price
// }
//
// fn verify_empty() {
// ::order.isInitialized shouldBe false
// }
// }
//
// class MockCartRemover : CartRemover {
//
// lateinit var id: CartId
//
// override fn deleteCart(cart: Cart) {
// this.id = cart.id
// }
//
// fn verify_invoked(cartId: CartId) {
// this.id shouldBe cartId
// }
//
// fn verify_empty() {
// ::id.isInitialized shouldBe false
// }
// }
//
// class MockCartExtractor : CartExtractor {
//
// lateinit var cart: Cart
// lateinit var forCustomer: CustomerId
//
// constructor()
// constructor(cart: Cart) {
// this.cart = cart
// }
//
// override fn getCart(forCustomer: CustomerId): Cart? {
// this.forCustomer = forCustomer
// return if (::cart.isInitialized) this.cart else null
// }
//
// fn verify_invoked(forCustomer: CustomerId) {
// this.forCustomer shouldBe forCustomer
// }
//
// fn verify_empty() {
// ::forCustomer.isInitialized shouldBe false
// }
// }
//
// class MockCustomerHasActiveOrder(val hasActive: Boolean) : CustomerHasActiveOrder {
//
// lateinit var forCustomer: CustomerId
//
// override fn invoke(forCustomer: CustomerId): Boolean {
// this.forCustomer = forCustomer
// return hasActive
// }
//
// fn verify_invoked(forCustomer: CustomerId) {
// this.forCustomer shouldBe forCustomer
// }
//
// fn verify_empty() {
// ::forCustomer.isInitialized shouldBe false
// }
// }

#[derive(new, Clone, PartialEq, Debug)]
pub struct TestMealExtractor<E: DomainEventTrait + Clone> {
    #[new(value = "HashMap::new()")]
    pub value: HashMap<MealId, Meal<E>>,
}

impl<E: DomainEventTrait + Clone> MealExtractor<E> for TestMealExtractor<E> {
    fn get_by_id(&mut self, id: MealId) -> Option<&Meal<E>> {
        self.value.get(&id)
    }

    fn get_by_name(&mut self, name: MealName) -> Option<Meal<E>> {
        let result = self
            .clone()
            .value
            .iter()
            .find_map(|(key, val)| if val.name == name { Some(key) } else { None })
            .and_then(|meal_id| self.get_by_id(*meal_id).cloned());
        result
    }

    fn get_all(&mut self) -> Vec<Meal<E>> {
        self.value.clone().into_values().collect()
    }
}

//
// class MockShopOrderExtractor : ShopOrderExtractor {
//
// lateinit var order: ShopOrder
//
// lateinit var id: ShopOrderId
// lateinit var forCustomer: CustomerId
// var all: Boolean = false
//
// constructor()
// constructor(order: ShopOrder) {
// this.order = order
// }
//
// override fn getById(orderId: ShopOrderId): ShopOrder? {
// this.id = orderId
// return if (::order.isInitialized && this.order.id == id) this.order else null
// }
//
// override fn getLastOrder(forCustomer: CustomerId): ShopOrder? {
// this.forCustomer = forCustomer
// return if (::order.isInitialized && this.order.forCustomer == forCustomer) this.order else null
// }
//
// override fn getAll(startId: ShopOrderId, limit: Int): List<ShopOrder> {
// this.all = true
// return if (::order.isInitialized) return listOf(this.order) else emptyList()
// }
//
// fn verify_invoked_get_by_id(id: ShopOrderId) {
// this.id shouldBe id
// this.all shouldBe false
// ::forCustomer.isInitialized shouldBe false
// }
//
// fn verifyInvokedGetLastOrder(forCustomer: CustomerId) {
// this.forCustomer shouldBe forCustomer
// this.all shouldBe false
// ::id.isInitialized shouldBe false
// }
//
// fn verify_invoked_get_all() {
// this.all shouldBe true
// ::id.isInitialized shouldBe false
// ::forCustomer.isInitialized shouldBe false
// }
//
// fn verify_empty() {
// this.all shouldBe false
// ::id.isInitialized shouldBe false
// ::forCustomer.isInitialized shouldBe false
// }
// }
//
// class MockOrderExporter : OrderExporter {
// lateinit var id: ShopOrderId
// lateinit var customerId: CustomerId
// lateinit var totalPrice: Price
//
// override fn exportOrder(id: ShopOrderId, customerId: CustomerId, totalPrice: Price) {
// this.id = id
// this.customerId = customerId
// this.totalPrice = totalPrice
// }
//
// fn verify_invoked(id: ShopOrderId, customerId: CustomerId, totalPrice: Price) {
// this.id shouldBe id
// this.customerId shouldBe customerId
// this.totalPrice shouldBe totalPrice
// }
// }
