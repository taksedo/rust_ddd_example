use std::{fmt, fmt::Formatter};

use common::types::{base::value_object::ValueObject, errors::error::BusinessError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Default)]
#[non_exhaustive]
pub struct MealName(String);

impl TryFrom<&str> for MealName {
    type Error = CreateMealNameError;

    fn try_from(value: &str) -> Result<MealName, Self::Error> {
        match value {
            x if x.is_empty() || x == " " => Err(Self::Error::EmptyMealNameError),
            _ => Ok(Self(value.to_string())),
        }
    }
}

impl ValueObject for MealName {}

#[derive(Debug, PartialEq)]
pub enum CreateMealNameError {
    EmptyMealNameError,
}

impl BusinessError for CreateMealNameError {}

impl fmt::Display for MealName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}
