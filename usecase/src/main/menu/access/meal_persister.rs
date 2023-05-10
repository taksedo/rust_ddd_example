use common_types::main::base::domain_event::DomainEventTrait;
use domain::main::menu::meal::Meal;
use domain::main::menu::meal_id::MealId;
use std::fmt::Debug;

pub trait MealPersister<E: DomainEventTrait>: Debug {
    fn save(&mut self, meal: Meal<E>);
}
