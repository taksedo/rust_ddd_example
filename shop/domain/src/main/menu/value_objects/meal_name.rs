use std::fmt;
use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

use common::types::main::base::value_object::ValueObject;
use common::types::main::errors::error::BusinessError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Default)]
#[non_exhaustive]
pub struct MealName {
    value: String,
}

impl TryFrom<&str> for MealName {
    type Error = CreateMealNameError;

    fn try_from(value: &str) -> Result<MealName, Self::Error> {
        match value {
            x if x.is_empty() || x == " " => Err(CreateMealNameError::EmptyMealNameError),
            _ => Ok(MealName {
                value: value.to_string(),
            }),
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
        write!(f, "{}", &self.value)
    }
}
