use std::fmt::Debug;

use crate::menu::value_objects::meal_name::MealName;

pub trait MealAlreadyExists: Debug + Send {
    fn invoke(&mut self, name: &MealName) -> bool;
}
