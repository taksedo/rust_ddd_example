use domain::main::menu::value_objects::meal_description::MealDescription;
use domain::main::menu::value_objects::meal_id::MealId;
use domain::main::menu::value_objects::meal_name::MealName;
use domain::main::menu::value_objects::price::Price;
use std::fmt::Debug;

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
