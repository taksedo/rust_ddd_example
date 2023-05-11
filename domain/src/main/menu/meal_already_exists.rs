use crate::main::menu::meal_name::MealName;

pub trait MealAlreadyExists {
    fn check(&mut self, name: MealName) -> bool;
}
