use bigdecimal::*;
use common::{common_rest::ValidationError, types::base::RCell};
use domain::menu::value_objects::{
    meal_description::{CreateMealDescriptionError, MealDescription},
    meal_id::{MealId, MealIdError},
    meal_name::{CreateMealNameError, MealName},
    price::{CreatePriceError, Price},
};

use crate::validated::Validated;

impl Validated<&str> for MealName {
    fn validated(val: &str, error_list: RCell<Vec<ValidationError>>) -> Option<Self> {
        match Self::try_from(val) {
            Ok(name) => Some(name),
            Err(CreateMealNameError::EmptyMealNameError) => {
                error_list
                    .borrow_mut()
                    .push(ValidationError::new("Meal name is empty."));
                None
            }
        }
    }
}

impl Validated<&str> for MealDescription {
    fn validated(val: &str, error_list: RCell<Vec<ValidationError>>) -> Option<Self> {
        match Self::try_from(val) {
            Ok(description) => Some(description),
            Err(CreateMealDescriptionError::EmptyDescriptionError) => {
                error_list
                    .borrow_mut()
                    .push(ValidationError::new("Meal description is empty"));
                None
            }
        }
    }
}

impl Validated<BigDecimal> for Price {
    fn validated(val: BigDecimal, error_list: RCell<Vec<ValidationError>>) -> Option<Self> {
        match Self::try_from(val) {
            Ok(price) => Some(price),
            Err(CreatePriceError::InvalidScale) => {
                error_list.borrow_mut().push(ValidationError::new(&format!(
                    "Price scale must not be > {}",
                    Price::SCALE
                )));
                None
            }
            Err(CreatePriceError::NegativeValue) => {
                error_list
                    .borrow_mut()
                    .push(ValidationError::new("Price must be > 0"));
                None
            }
        }
    }
}

impl Validated<i64> for MealId {
    fn validated(val: i64, error_list: RCell<Vec<ValidationError>>) -> Option<Self> {
        match Self::try_from(val) {
            Ok(id) => Some(id),
            Err(MealIdError::IdGenerationError) => {
                error_list
                    .borrow_mut()
                    .push(ValidationError::new("Meal Id must be > 0"));
                None
            }
        }
    }
}
