use crate::main::menu::dto::meal_info::MealInfo;
use domain::main::menu::meal_id::MealId;

pub trait GetMealById {
    fn execute(&mut self, id: MealId) -> Result<MealInfo, GetMealByIdUseCaseError>;
}

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum GetMealByIdUseCaseError {
    #[error("Еда не найдена")]
    MealNotFound,
}
