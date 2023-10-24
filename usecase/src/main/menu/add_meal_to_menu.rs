use actix_web::error::ResponseError;
use domain::main::menu::value_objects::meal_description::MealDescription;
use domain::main::menu::value_objects::meal_id::MealId;
use domain::main::menu::value_objects::meal_name::MealName;
use domain::main::menu::value_objects::price::Price;
use std::fmt::Debug;
use thiserror::Error;

pub trait AddMealToMenu: Debug + Send {
    fn execute(
        &mut self,
        name: MealName,
        description: MealDescription,
        price: Price,
    ) -> Result<MealId, AddMealToMenuUseCaseError>;
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum AddMealToMenuUseCaseError {
    #[error("Неверные параметры еды")]
    InvalidParameters,
    #[error("Еда с таким именем уже существует")]
    AlreadyExists,
}

impl ResponseError for AddMealToMenuUseCaseError {}
