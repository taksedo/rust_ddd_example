use std::any::type_name;

use common::events::main::domain_event_publisher::DomainEventPublisher;
use derive_new::new;

use domain::main::menu::meal::Meal;
use domain::main::menu::meal_events::MealEventEnum;
use domain::test_fixtures::rnd_meal;

pub fn meal_with_events() -> Meal {
    let mut meal = rnd_meal();
    meal.remove_meal_from_menu();
    meal
}

#[derive(new, Clone, Debug)]
pub struct TestEventPublisher {
    #[new(value = "vec![]")]
    pub storage: Vec<MealEventEnum>,
}

impl DomainEventPublisher<MealEventEnum> for TestEventPublisher {
    fn publish(&mut self, events: &Vec<MealEventEnum>) {
        self.storage.extend(events.clone());
    }
}

pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
