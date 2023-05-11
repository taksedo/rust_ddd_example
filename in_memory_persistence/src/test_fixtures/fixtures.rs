use common_events::main::domain_event_publisher::DomainEventPublisher;
use common_types::main::base::domain_event::DomainEventTrait;
use derive_new::new;
use domain::main::menu::meal::Meal;
use domain::main::menu::meal_events::MealRemovedFromMenuDomainEvent;
use domain::test_fixtures::fixtures::rnd_meal;
use std::any::type_name;

pub fn meal_with_events() -> Meal<MealRemovedFromMenuDomainEvent> {
    let mut meal = rnd_meal();
    meal.remove_meal_from_menu();
    meal
}

#[derive(new, Debug, Clone)]
pub struct TestEventPublisher<E: DomainEventTrait> {
    #[new(value = "vec![]")]
    pub storage: Vec<E>,
}

impl<E: DomainEventTrait + Clone> DomainEventPublisher<E> for TestEventPublisher<E> {
    fn publish(&mut self, events: &Vec<E>) {
        self.storage.extend(events.clone());
    }
}

pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
