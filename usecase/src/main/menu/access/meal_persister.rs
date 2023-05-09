use domain::main::menu::meal::Meal;
use domain::main::menu::meal_id::MealId;
use std::fmt::Debug;

pub trait MealPersister: Debug {
    fn save(&mut self, meal: Meal);
    fn get_meal_by_id(&self, id: &MealId) -> Option<&Meal>;
}
