use std::any::type_name;
use std::fmt::Debug;

use common::events::main::domain_event_publisher::DomainEventPublisher;
use derive_new::new;

use domain::main::cart::cart::Cart;
use domain::main::menu::meal::Meal;
use domain::test_fixtures::{rnd_cart, rnd_meal};

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
