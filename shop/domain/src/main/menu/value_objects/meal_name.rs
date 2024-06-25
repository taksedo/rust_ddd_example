use common::types::{base::value_object::ValueObject, errors::error::BusinessError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Default, Display)]
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

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn create_name__success() {
        let value = "Some string";
        let result = MealName::try_from(value);

        let meal_name = result.unwrap();
        assert_eq!(meal_name.to_string(), value);
    }

    #[rstest]
    fn create_name__empty_string(#[values("", " ")] input: &str) {
        let result = MealName::try_from(input);
        assert_eq!(result, Err(CreateMealNameError::EmptyMealNameError));
    }
}
