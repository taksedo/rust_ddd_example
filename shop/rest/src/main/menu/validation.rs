use std::sync::{Arc, Mutex};

use bigdecimal::*;
use common::common_rest::rest_responses::ValidationError;
use domain::main::menu::value_objects::{
    meal_description::{CreateMealDescriptionError, MealDescription},
    meal_id::{MealId, MealIdError},
    meal_name::{CreateMealNameError, MealName},
    price::{CreatePriceError, Price},
};

use crate::main::validated::Validated;

impl Validated<MealName, &str> for MealName {
    fn validated(val: &str, error_list: Arc<Mutex<Vec<ValidationError>>>) -> Result<Self, ()> {
        Self::try_from(val).map_err(|e| match e {
            CreateMealNameError::EmptyMealNameError => error_list
                .lock()
                .unwrap()
                .push(ValidationError::new("Meal name is empty.")),
        })
    }
}

impl Validated<MealDescription, &str> for MealDescription {
    fn validated(val: &str, error_list: Arc<Mutex<Vec<ValidationError>>>) -> Result<Self, ()> {
        Self::try_from(val).map_err(|e| match e {
            CreateMealDescriptionError::EmptyDescriptionError => error_list
                .lock()
                .unwrap()
                .push(ValidationError::new("Meal description is empty")),
        })
    }
}

impl Validated<Price, BigDecimal> for Price {
    fn validated(
        val: BigDecimal,
        error_list: Arc<Mutex<Vec<ValidationError>>>,
    ) -> Result<Self, ()> {
        Self::try_from(val).map_err(|e| match e {
            CreatePriceError::InvalidScale => {
                error_list
                    .lock()
                    .unwrap()
                    .push(ValidationError::new(&format!(
                        "Price scale must not be > {}",
                        Price::SCALE
                    )))
            }
            CreatePriceError::NegativeValue => error_list
                .lock()
                .unwrap()
                .push(ValidationError::new("Price must be > 0")),
        })
    }
}

impl Validated<MealId, i64> for MealId {
    fn validated(val: i64, error_list: Arc<Mutex<Vec<ValidationError>>>) -> Result<Self, ()> {
        Self::try_from(val).map_err(|e| match e {
            MealIdError::IdGenerationError => error_list
                .lock()
                .unwrap()
                .push(ValidationError::new("Meal Id must be > 0")),
        })
    }
}
