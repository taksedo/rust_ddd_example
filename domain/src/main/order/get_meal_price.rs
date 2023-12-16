use std::fmt::Debug;

use crate::main::menu::value_objects::{meal_id::MealId, price::Price};

pub trait GetMealPrice: Debug + Send {
    fn invoke(&self, for_meal_id: MealId) -> Price;
}
