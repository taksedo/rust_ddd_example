use std::{any::type_name, fmt::Debug};

#[path = "./domain.rs"]
mod domain_test_fixtures;

#[path = "./usecase.rs"]
mod usecase_test_fixtures;

use common::events::domain_event_publisher::DomainEventPublisher;
use derive_new::new;
use domain::{cart::cart::Cart, menu::meal::Meal, order::shop_order::ShopOrder};
use domain_test_fixtures::{rnd_cart, rnd_meal};
use usecase_test_fixtures::order_ready_for_complete;

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
