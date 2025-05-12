use std::{
    any::{Any, TypeId},
    collections::HashMap,
    mem::discriminant,
};

use common::types::common::{Address, Count};
use derive_new::new;
use domain::{
    cart::{
        cart::Cart,
        value_objects::{cart_id::CartId, customer_id::CustomerId},
    },
    menu::{
        meal::Meal,
        meal_events::MealEventEnum,
        value_objects::{
            meal_description::MealDescription, meal_id::MealId, meal_name::MealName, price::Price,
        },
    },
    order::{
        customer_has_active_order::CustomerHasActiveOrder,
        customer_order_events::{
            ShopOrderCancelledDomainEvent, ShopOrderCompletedDomainEvent,
            ShopOrderConfirmedDomainEvent, ShopOrderEventEnum, ShopOrderPaidDomainEvent,
        },
        shop_order::{OrderState, ShopOrder},
        value_objects::shop_order_id::ShopOrderId,
    },
    test_fixtures::{order_with_state, rnd_meal},
};

use crate::{
    cart::access::{
        cart_extractor::CartExtractor, cart_persister::CartPersister, cart_remover::CartRemover,
    },
    menu::access::{meal_extractor::MealExtractor, meal_persister::MealPersister},
    order::{
        access::{
            shop_order_extractor::ShopOrderExtractor, shop_order_persister::ShopOrderPersister,
        },
        providers::order_exporter::OrderExporter,
    },
};

pub fn removed_meal() -> Meal {
    let mut meal = rnd_meal();
    meal.remove_meal_from_menu();
    meal
}

pub fn order_ready_for_pay() -> ShopOrder {
    order_with_state(OrderState::new_waiting_for_payment())
}

pub fn order_not_ready_for_pay() -> ShopOrder {
    order_with_state(OrderState::new_completed())
}

pub fn order_ready_for_cancel() -> ShopOrder {
    order_with_state(OrderState::new_paid())
}

pub fn order_not_ready_for_cancel() -> ShopOrder {
    order_with_state(OrderState::new_completed())
}

pub fn order_ready_for_confirm() -> ShopOrder {
    order_with_state(OrderState::new_paid())
}

pub fn order_not_ready_for_confirm() -> ShopOrder {
    order_with_state(OrderState::new_waiting_for_payment())
}

pub fn order_ready_for_complete() -> ShopOrder {
    order_with_state(OrderState::new_confirmed())
}

pub fn order_not_ready_for_complete() -> ShopOrder {
    order_with_state(OrderState::new_cancelled())
}

pub fn active_order() -> ShopOrder {
    order_with_state(OrderState::new_confirmed())
}

pub fn non_active_order() -> ShopOrder {
    order_with_state(OrderState::new_cancelled())
}

#[derive(new, Debug, Clone)]
pub struct MockMealPersister {
    #[new(value = "None")]
    pub meal: Option<Meal>,
}

impl MockMealPersister {
    pub fn verify_invoked(
        &self,
        id: Option<&MealId>,
        name: Option<&MealName>,
        description: Option<&MealDescription>,
        price: Option<&Price>,
    ) {
        let meal = &self.meal.clone().unwrap();
        if id.is_some() {
            assert_eq!(meal.id(), id.unwrap())
        }
        if name.is_some() {
            assert_eq!(meal.name(), name.unwrap())
        }
        if description.is_some() {
            assert_eq!(meal.description(), description.unwrap())
        }
        if price.is_some() {
            assert_eq!(meal.price(), price.unwrap())
        }
    }

    pub fn verify_invoked_meal(&self, meal: &Meal) {
        assert_eq!(&self.meal.clone().unwrap(), meal)
    }

    pub fn verify_events_after_deletion(&mut self, _id: &MealId) {
        let events = self
            .to_owned()
            .meal
            .unwrap()
            .pop_events()
            .first()
            .unwrap()
            .clone();
        assert_eq!(events.type_id(), TypeId::of::<MealEventEnum>());
    }

    pub fn verify_empty(&self) {
        assert!(&self.meal.is_none());
    }
}

impl MealPersister for MockMealPersister {
    fn save(&mut self, meal: Meal) {
        self.meal = Some(meal);
    }
}

#[derive(new, Clone, PartialEq, Debug, Default)]
pub struct MockMealExtractor {
    #[new(default)]
    pub meal: Option<Meal>,
    #[new(default)]
    pub id: Option<MealId>,
    #[new(default)]
    pub name: Option<MealName>,
    #[new(default)]
    pub all: bool,
}

impl MealExtractor for MockMealExtractor {
    fn get_by_id(&mut self, id: &MealId) -> Option<Meal> {
        self.id = Option::from(*id);
        if Some(&self.meal).is_some() && self.id == Some(*id) {
            self.clone().meal
        } else {
            None
        }
    }

    fn get_by_name(&mut self, name: &MealName) -> Option<Meal> {
        self.name = Option::from(name.to_owned());
        if Some(&self.meal).is_some() && &self.to_owned().name.unwrap() == name {
            self.to_owned().meal
        } else {
            None
        }
    }

    fn get_all(&mut self) -> Vec<Meal> {
        self.all = true;
        if self.meal.is_some() {
            vec![self.to_owned().meal.unwrap().clone()]
        } else {
            vec![]
        }
    }
}

impl MockMealExtractor {
    pub fn verify_invoked_get_by_id(&self, id: &MealId) {
        assert_eq!(&self.id.unwrap(), id);
        assert!(!&self.all);
        assert!(&self.name.is_none());
    }

    pub fn verify_invoked_get_by_name(&self, name: &MealName) {
        assert_eq!(&self.clone().name.unwrap(), name);
        assert!(!&self.all);
        assert!(&self.id.is_none());
    }

    pub fn verify_invoked_get_all(&self) {
        assert!(&self.all);
        assert!(&self.id.is_none());
        assert!(&self.name.is_none());
    }

    pub fn verify_empty(&self) {
        assert!(&self.name.is_none());
    }
}

#[derive(new, Debug, Clone, Default)]
pub struct MockCartPersister {
    pub cart: Option<Cart>,
}

impl MockCartPersister {
    pub fn verify_invoked(
        &self,
        cart: Option<&Cart>,
        cart_id: Option<&CartId>,
        meal_id: Option<&MealId>,
        customer_id: Option<&CustomerId>,
    ) {
        let self_cart = &self.cart.clone().unwrap();
        if cart.is_some() {
            assert_eq!(self_cart, cart.unwrap());
        }
        if cart_id.is_some() {
            assert_eq!(self_cart.id(), cart_id.unwrap());
        }
        if meal_id.is_some() {
            assert_eq!(
                self_cart.meals(),
                &HashMap::from([(*meal_id.unwrap(), Count::one())])
            );
        }
        if customer_id.is_some() {
            assert_eq!(self_cart.for_customer(), customer_id.unwrap());
        }
    }

    pub fn verify_empty(&self) {
        assert!(&self.cart.is_none())
    }
}

impl CartPersister for MockCartPersister {
    fn save(&mut self, cart: Cart) {
        self.cart = Some(cart);
    }
}

#[derive(new, Clone, PartialEq, Debug, Default)]
pub struct MockCartRemover {
    pub id: Option<CartId>,
}

impl MockCartRemover {
    pub fn verify_invoked(&self, cart_id: &CartId) {
        assert_eq!(&self.id.unwrap(), cart_id)
    }

    pub fn verify_empty(&self) {
        assert!(&self.id.is_none())
    }
}

impl CartRemover for MockCartRemover {
    fn delete_cart(&mut self, cart: Cart) {
        self.id = Some(*cart.id());
    }
}

#[derive(new, Clone, PartialEq, Debug, Default)]
pub struct MockCartExtractor {
    pub cart: Option<Cart>,
    pub for_customer: Option<CustomerId>,
}

impl CartExtractor for MockCartExtractor {
    fn get_cart(&mut self, for_customer: &CustomerId) -> Option<Cart> {
        self.for_customer = Some(*for_customer);
        self.cart.as_ref().map(ToOwned::to_owned)
    }
}

impl MockCartExtractor {
    pub fn verify_invoked(&self, for_customer: &CustomerId) {
        assert_eq!(&self.for_customer.unwrap(), for_customer)
    }

    pub fn verify_empty(&self) {
        assert!(&self.for_customer.is_none())
    }
}

#[derive(new, Clone, PartialEq, Debug, Default)]
pub struct MockShopOrderExtractor {
    pub order: Option<ShopOrder>,
    pub id: Option<ShopOrderId>,
    pub for_customer: Option<CustomerId>,
    pub all: bool,
}

impl ShopOrderExtractor for MockShopOrderExtractor {
    fn get_by_id(&mut self, order_id: &ShopOrderId) -> Option<ShopOrder> {
        self.id = Some(*order_id);
        if self.order.is_some() && self.order.clone().unwrap().id() == &self.id.unwrap() {
            self.order.as_ref().cloned()
        } else {
            None
        }
    }

    fn get_last_order(&mut self, for_customer: &CustomerId) -> Option<ShopOrder> {
        self.for_customer = Some(*for_customer);
        if self.order.is_some() && self.order.clone()?.for_customer() == for_customer {
            self.order.as_ref().cloned()
        } else {
            None
        }
    }

    fn get_all(&mut self, _start_id: &ShopOrderId, _limit: usize) -> Vec<ShopOrder> {
        self.all = true;
        if self.order.is_some() {
            vec![self.order.clone().unwrap()]
        } else {
            vec![]
        }
    }
}

impl MockShopOrderExtractor {
    pub fn verify_invoked_get_by_id(&self, id: &ShopOrderId) {
        assert_eq!(self.id, Some(*id));
        assert!(!self.all);
        assert!(self.for_customer.is_none());
    }

    pub fn verify_invoked_get_last_order(&self, for_customer: &CustomerId) {
        assert_eq!(self.for_customer, Some(*for_customer));
        assert!(!self.all);
        assert!(self.id.is_none());
    }

    pub fn verify_invoked_get_all(&self) {
        assert!(self.all);
        assert!(self.id.is_none());
        assert!(self.for_customer.is_none());
    }

    pub fn verify_empty(&self) {
        assert!(!self.all);
        assert!(self.id.is_none());
        assert!(self.for_customer.is_none());
    }
}

#[derive(new, Clone, PartialEq, Debug, Default)]
pub struct MockShopOrderPersister {
    pub order: Option<ShopOrder>,
}

impl ShopOrderPersister for MockShopOrderPersister {
    fn save(&mut self, order: ShopOrder) {
        self.order = Some(order);
    }
}

impl MockShopOrderPersister {
    pub fn verify_invoked_order(&self, order: &ShopOrder) {
        assert_eq!(self.order.clone().unwrap(), order.clone());
    }

    pub fn verify_invoked(
        &self,
        order_id: &ShopOrderId,
        address: &Address,
        customer_id: &CustomerId,
        meal_id: &MealId,
        count_items: &Count,
        price_items: &Price,
    ) {
        let order = self.order.clone().unwrap();
        assert_eq!(order.id(), order_id);
        assert_eq!(order.address(), address);
        assert_eq!(order.for_customer(), customer_id);
        assert_eq!(order.order_items().len(), 1);

        let order_item = order.order_items().iter().next().unwrap();
        assert_eq!(order_item.meal_id, *meal_id);
        assert_eq!(order_item.count, *count_items);
        assert_eq!(order_item.price, *price_items);
    }
    pub fn verify_events_after_cancellation(&self, id: &ShopOrderId) {
        let events = self.order.clone().unwrap().pop_events();
        let first_event = events.first().unwrap().clone();
        let etalon_event = ShopOrderCancelledDomainEvent::new(*id);
        assert_eq!(events.len(), 1);
        assert_eq!(
            discriminant(&Into::<ShopOrderEventEnum>::into(first_event.clone())),
            discriminant(&Into::<ShopOrderEventEnum>::into(etalon_event))
        );
        let first_event_struct: ShopOrderCancelledDomainEvent = first_event.try_into().unwrap();
        assert_eq!(first_event_struct.order_id, *id);
    }
    pub fn verify_events_after_completion(&mut self, id: &ShopOrderId) {
        let events = self.order.clone().unwrap().pop_events();
        let first_event = events.first().unwrap().clone();
        let etalon_event = ShopOrderCompletedDomainEvent::new(*id);
        assert_eq!(events.len(), 1);
        assert_eq!(
            discriminant(&Into::<ShopOrderEventEnum>::into(first_event.clone())),
            discriminant(&Into::<ShopOrderEventEnum>::into(etalon_event))
        );
        let first_event_struct: ShopOrderCompletedDomainEvent = first_event.try_into().unwrap();
        assert_eq!(first_event_struct.order_id, *id);
    }

    pub fn verify_events_after_confirmation(&mut self, id: &ShopOrderId) {
        let events = self.order.clone().unwrap().pop_events();
        let first_event = events.first().unwrap().clone();
        let etalon_event = ShopOrderConfirmedDomainEvent::new(*id);
        assert_eq!(events.len(), 1);
        assert_eq!(
            discriminant(&Into::<ShopOrderEventEnum>::into(first_event.clone())),
            discriminant(&Into::<ShopOrderEventEnum>::into(etalon_event))
        );
        let first_event_struct: ShopOrderConfirmedDomainEvent = first_event.try_into().unwrap();
        assert_eq!(first_event_struct.order_id, *id);
    }

    pub fn verify_events_after_payment(&mut self, id: &ShopOrderId) {
        let events = self.order.clone().unwrap().pop_events();
        let first_event = events.first().unwrap().clone();
        let etalon_event = ShopOrderPaidDomainEvent::new(*id);
        assert_eq!(events.len(), 1);
        assert_eq!(
            discriminant(&Into::<ShopOrderEventEnum>::into(first_event.clone())),
            discriminant(&Into::<ShopOrderEventEnum>::into(etalon_event))
        );
        let first_event_struct: ShopOrderPaidDomainEvent = first_event.try_into().unwrap();
        assert_eq!(first_event_struct.order_id, *id);
    }

    pub fn verify_price(&self, price: &Price) {
        assert_eq!(self.order.clone().unwrap().total_price(), *price);
    }

    pub fn verify_empty(&self) {
        assert!(self.order.is_none());
    }
}

#[derive(new, Clone, Eq, PartialEq, Debug, Default)]
pub struct MockOrderExporter {
    pub id: ShopOrderId,
    pub customer_id: CustomerId,
    pub total_price: Price,
}

impl OrderExporter for MockOrderExporter {
    fn export_order(&mut self, id: ShopOrderId, customer_id: CustomerId, total_price: Price) {
        self.id = id;
        self.customer_id = customer_id;
        self.total_price = total_price;
    }
}

impl MockOrderExporter {
    pub fn verify_invoked(&self, id: ShopOrderId, customer_id: CustomerId, total_price: Price) {
        assert_eq!(self.id, id);
        assert_eq!(self.customer_id, customer_id);
        assert_eq!(self.total_price, total_price);
    }
}

#[derive(new, Debug, Default)]
pub struct MockCustomerHasActiveOrder {
    pub has_active: bool,
    #[new(value = "Default::default()")]
    pub for_customer: Option<CustomerId>,
}

impl MockCustomerHasActiveOrder {
    pub fn verify_invoked(&self, for_customer: &CustomerId) {
        assert_eq!(&self.for_customer.unwrap(), for_customer);
    }

    pub fn verify_empty(&self) {
        assert!(&self.for_customer.is_none())
    }
}

impl CustomerHasActiveOrder for MockCustomerHasActiveOrder {
    fn invoke(&mut self, for_customer: &CustomerId) -> bool {
        self.for_customer = Some(*for_customer);
        self.has_active
    }
}
