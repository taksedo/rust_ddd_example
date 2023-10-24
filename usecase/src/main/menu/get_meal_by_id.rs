use crate::main::menu::dto::meal_info::MealInfo;
use actix_web::ResponseError;
use domain::main::menu::value_objects::meal_id::MealId;

pub trait GetMealById {
    fn execute(&mut self, id: MealId) -> Result<MealInfo, GetMealByIdUseCaseError>;
}

#[derive(thiserror::Error, Debug, PartialEq, Clone, Copy)]
pub enum GetMealByIdUseCaseError {
    #[error("Еда не найдена")]
    MealNotFound,
}

impl ResponseError for GetMealByIdUseCaseError {}
