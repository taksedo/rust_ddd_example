use common::types::{base::ValueObject, errors::BusinessError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize, Display)]
#[non_exhaustive]
pub struct MealDescription(String);

impl TryFrom<&str> for MealDescription {
    type Error = CreateMealDescriptionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            x if x.is_empty() || x == " " => Err(Self::Error::EmptyDescriptionError),
            _ => Ok(Self(value.to_string())),
        }
    }
}

impl ValueObject for MealDescription {}

#[derive(Debug, PartialEq)]
pub enum CreateMealDescriptionError {
    EmptyDescriptionError,
}

impl BusinessError for CreateMealDescriptionError {}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use std::any::type_name;

    use mockall::Any;
    use rstest::rstest;

    use super::*;

    #[test]
    fn create_description__success() {
        let value = "Some string";
        let result = MealDescription::try_from(value).unwrap();

        let meal_type_name = result.type_name();
        assert_eq!(meal_type_name, type_name::<MealDescription>());
        let description = result;
        assert_eq!(description.to_string(), value);
    }

    #[rstest]
    fn create_description__empty_string(#[values("", " ")] input: &str) {
        let result = MealDescription::try_from(input);
        assert_eq!(
            result,
            Err(CreateMealDescriptionError::EmptyDescriptionError)
        );
    }
}
