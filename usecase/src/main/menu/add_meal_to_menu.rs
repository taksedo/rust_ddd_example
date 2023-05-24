use domain::main::menu::meal_id::MealId;
use domain::main::menu::meal_name::MealName;
use std::fmt::Debug;
use thiserror::Error;

pub trait AddMealToMenu: Debug + Send {
    fn execute(&mut self, name: MealName) -> Result<MealId, AddMealToMenuUseCaseError>;
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum AddMealToMenuUseCaseError {
    #[error("Неверные параметры еды")]
    InvalidParameters,
    #[error("Еда с таким именем уже существует")]
    AlreadyExists,
}
