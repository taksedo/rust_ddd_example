use derive_new::new;
use domain::main::menu::meal_id::MealId;
use domain::main::menu::meal_name::MealName;

pub trait AddMealToMenu {
    fn execute(
        &mut self,
        request: AddMealToMenuRequest,
    ) -> Result<MealId, AddMealToMenuUseCaseError>;
}

#[derive(thiserror::Error, Debug)]
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
        let meal_name = MealName::from(name);
        match meal_name {
            Ok(meal_name) => Ok(AddMealToMenuRequest::new(meal_name)),
            Err(_) => Err(InvalidMealParametersError::InvalidParameters),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum InvalidMealParametersError {
    #[error("Неверные параметры еды")]
    InvalidParameters,
}
