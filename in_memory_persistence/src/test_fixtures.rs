use std::any::type_name;
use std::fmt::Debug;

use common::events::main::domain_event_publisher::DomainEventPublisher;
use derive_new::new;

use domain::main::cart::cart::Cart;
use domain::main::menu::meal::Meal;
use domain::main::order::shop_order::ShopOrder;
use domain::test_fixtures::{rnd_cart, rnd_meal};
use usecase::test_fixtures::order_ready_for_complete;

pub fn meal_with_events() -> Meal {
    let mut meal = rnd_meal();
    meal.remove_meal_from_menu();
    meal
}

pub fn cart_with_events() -> Cart {
    let mut cart = rnd_cart();
    cart.add_meal(rnd_meal());
    cart
}

pub fn order_with_events() -> ShopOrder {
    let mut order = order_ready_for_complete();

    assert!(order.complete().is_ok());
    order
}

#[derive(new, Clone, Debug)]
pub struct TestEventPublisher<Event> {
    #[new(value = "vec![]")]
    pub storage: Vec<Event>,
}

impl<Event: Debug + Send + Clone> DomainEventPublisher<Event> for TestEventPublisher<Event> {
    fn publish(&mut self, events: &Vec<Event>) {
        events.iter().for_each(|it| self.storage.push(it.clone()))
    }
}

pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
