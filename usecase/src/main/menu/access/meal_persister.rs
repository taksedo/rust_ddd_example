use domain::main::menu::meal::Meal;
use std::fmt::Debug;

pub trait MealPersister: Debug + Send {
    fn save(&mut self, meal: Meal);
}
