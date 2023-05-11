use common_types::main::base::domain_event::DomainEventTrait;
use domain::main::menu::meal::Meal;
use std::fmt::Debug;

pub trait MealPersister<E: DomainEventTrait + Clone>: Debug {
    fn save(&mut self, meal: Meal<E>);
}
