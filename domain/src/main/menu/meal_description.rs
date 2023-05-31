use actix_web::ResponseError;
use common_types::main::base::value_object::ValueObject;
use common_types::main::errors::error::BusinessError;
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, new, PartialEq, Eq, Default, Deserialize, Serialize)]
pub struct MealDescription {
    value: String,
}

impl MealDescription {
    pub fn from(description: String) -> Result<MealDescription, CreateMealDescriptionError> {
        match description {
            x if x == *"" || x == *" " => Err(CreateMealDescriptionError::EmptyDescriptionError),
            _ => Ok(Self::new(description)),
        }
    }
}

impl ValueObject for MealDescription {}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum CreateMealDescriptionError {
    #[error("Описание еды пустое")]
    EmptyDescriptionError,
}

impl BusinessError for CreateMealDescriptionError {}

impl ResponseError for CreateMealDescriptionError {}

impl fmt::Display for MealDescription {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.value)
    }
}
