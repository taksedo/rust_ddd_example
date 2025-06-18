#![allow(dead_code)]
use std::fmt::Debug;

use async_trait::async_trait;
use common::{events::DomainEventPublisher, types::base::DomainEventTrait};
use derive_new::new;
use domain::{cart::cart::Cart, menu::meal::Meal, order::shop_order::ShopOrder, test_fixtures::*};
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

#[async_trait]
impl<Event: Debug + Send + Clone + DomainEventTrait + Sync> DomainEventPublisher<Event>
    for TestEventPublisher<Event>
{
    async fn publish(&mut self, events: &[Event]) {
        events.iter().for_each(|it| self.storage.push(it.clone()))
    }
}
