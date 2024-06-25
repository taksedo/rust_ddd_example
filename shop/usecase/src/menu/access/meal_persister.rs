use std::fmt::Debug;

use domain::menu::meal::Meal;

pub trait MealPersister: Debug + Send {
    fn save(&mut self, meal: Meal);
}
