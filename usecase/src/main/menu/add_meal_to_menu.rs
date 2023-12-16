use std::fmt::Debug;

use domain::main::menu::value_objects::{
    meal_description::MealDescription, meal_id::MealId, meal_name::MealName, price::Price,
};

pub trait AddMealToMenu: Debug + Send {
    fn execute(
        &mut self,
        name: MealName,
        description: MealDescription,
        price: Price,
    ) -> Result<MealId, AddMealToMenuUseCaseError>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum AddMealToMenuUseCaseError {
    InvalidParameters,
    AlreadyExists,
    UnknownError,
}
