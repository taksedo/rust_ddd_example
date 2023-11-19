use crate::main::menu::dto::meal_info::MealInfo;
use domain::main::menu::value_objects::meal_id::MealId;

pub trait GetMealById {
    fn execute(&mut self, id: MealId) -> Result<MealInfo, GetMealByIdUseCaseError>;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GetMealByIdUseCaseError {
    MealNotFound,
}
