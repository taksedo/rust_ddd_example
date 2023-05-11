use common_types::main::base::domain_event::DomainEventTrait;
use domain::main::menu::meal::Meal;
use domain::main::menu::meal_id::MealId;
use domain::main::menu::meal_name::MealName;
use std::fmt::Debug;

pub trait MealExtractor<E: DomainEventTrait + Clone>: Debug {
    fn get_by_id(&mut self, id: MealId) -> Option<&Meal<E>>;

    fn get_by_name(&mut self, name: MealName) -> Option<Meal<E>>;

    fn get_all(&mut self) -> Vec<Meal<E>>;
}
