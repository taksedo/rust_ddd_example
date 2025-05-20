use std::fmt::Debug;

use async_trait::async_trait;
use domain::menu::value_objects::meal_id::MealId;

use crate::menu::dto::meal_info::MealInfo;

#[async_trait]
pub trait GetMealById {
    async fn execute(&mut self, id: &MealId) -> Result<MealInfo, GetMealByIdUseCaseError>;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GetMealByIdUseCaseError {
    MealNotFound,
}
