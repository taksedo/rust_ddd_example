use crate::main::menu::value_objects::meal_id::MealId;
use crate::main::menu::value_objects::price::Price;

pub trait GetMealPrice {
    fn invoke(&self, for_meal_id: MealId) -> Price;
}
