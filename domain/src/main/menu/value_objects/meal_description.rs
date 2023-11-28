use std::fmt;
use std::fmt::Formatter;

use common::types::main::base::value_object::ValueObject;
use common::types::main::errors::error::BusinessError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
#[non_exhaustive]
pub struct MealDescription {
    value: String,
}

impl TryFrom<&str> for MealDescription {
    type Error = CreateMealDescriptionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            x if x.is_empty() || x == " " => Err(CreateMealDescriptionError::EmptyDescriptionError),
            _ => Ok(MealDescription {
                value: value.to_string(),
            }),
        }
    }
}

impl ValueObject for MealDescription {}

#[derive(Debug, PartialEq)]
pub enum CreateMealDescriptionError {
    EmptyDescriptionError,
}

impl BusinessError for CreateMealDescriptionError {}

impl fmt::Display for MealDescription {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.value)
    }
}
