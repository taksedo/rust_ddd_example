use actix_web::ResponseError;
use common_types::main::base::value_object::ValueObject;
use common_types::main::errors::error::BusinessError;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Default, new)]
pub struct MealName {
    pub value: String,
}

impl MealName {
    pub fn to_string_value(&self) -> String {
        self.value.clone()
    }

    pub fn from(name: String) -> Result<Self, CreateMealNameError> {
        match name {
            x if x == *"" || x == *" " => Err(CreateMealNameError::EmptyMealNameError),
            _ => Ok(Self::new(name)),
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

impl ResponseError for CreateMealNameError {}
