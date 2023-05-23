use common_types::main::base::value_object::ValueObject;
use common_types::main::errors::error::BusinessError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Default)]
pub struct MealName {
    pub value: String,
}

impl MealName {
    pub fn to_string_value(&self) -> &String {
        &self.value
    }

    pub fn from(name: String) -> Result<Self, CreateMealNameError> {
        if name == *"" || name == *" " {
            Err(CreateMealNameError::EmptyMealNameError)
        } else {
            Ok(Self { value: name })
        }
    }
}

impl ValueObject for MealName {}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum CreateMealNameError {
    #[error("Название еды пустое")]
    EmptyMealNameError,
}

impl BusinessError for CreateMealNameError {}
