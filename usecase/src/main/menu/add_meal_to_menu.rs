use derive_new::new;
use domain::main::menu::meal_id::MealId;
use domain::main::menu::meal_name::MealName;
use thiserror::Error;

pub trait AddMealToMenu {
    fn execute(
        &mut self,
        request: AddMealToMenuRequest,
    ) -> Result<MealId, AddMealToMenuUseCaseError>;
}

#[derive(Error, Debug)]
pub enum AddMealToMenuUseCaseError {
    #[error("Еда с таким именем уже существует")]
    AlreadyExists,
}

#[derive(new, PartialEq, Debug)]
pub struct AddMealToMenuRequest {
    pub name: MealName,
}

impl AddMealToMenuRequest {
    pub fn from(name: String) -> Result<AddMealToMenuRequest, InvalidMealParametersError> {
        MealName::from(name)
            .map_err(|_| InvalidMealParametersError::InvalidParameters)
            .map(AddMealToMenuRequest::new)
    }
}

#[derive(Error, Debug)]
pub enum InvalidMealParametersError {
    #[error("Неверные параметры еды")]
    InvalidParameters,
}
