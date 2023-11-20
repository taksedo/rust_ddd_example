use std::sync::{Arc, Mutex};

use bigdecimal::*;

use common_rest::main::rest_responses::ValidationError;
use domain::main::menu::value_objects::meal_description::{
    CreateMealDescriptionError, MealDescription,
};
use domain::main::menu::value_objects::meal_name::{CreateMealNameError, MealName};
use domain::main::menu::value_objects::price::{CreatePriceError, Price};

pub trait Validated<Entity, ValueType> {
    #[allow(clippy::result_unit_err)]
    fn validated(
        val: ValueType,
        error_list: Arc<Mutex<Vec<ValidationError>>>,
    ) -> Result<Entity, ()>;
}

impl Validated<MealName, &str> for MealName {
    fn validated(val: &str, error_list: Arc<Mutex<Vec<ValidationError>>>) -> Result<Self, ()> {
        Self::try_from(val).map_err(|e| match e {
            CreateMealNameError::EmptyMealNameError => error_list
                .lock()
                .unwrap()
                .push(ValidationError::new("Meal name is empty.".to_string())),
        })
    }
}

impl Validated<MealDescription, &str> for MealDescription {
    fn validated(val: &str, error_list: Arc<Mutex<Vec<ValidationError>>>) -> Result<Self, ()> {
        Self::try_from(val).map_err(|e| match e {
            CreateMealDescriptionError::EmptyDescriptionError => error_list.lock().unwrap().push(
                ValidationError::new("Meal description is empty".to_string()),
            ),
        })
    }
}

impl Validated<Price, BigDecimal> for Price {
    fn validated(
        val: BigDecimal,
        error_list: Arc<Mutex<Vec<ValidationError>>>,
    ) -> Result<Self, ()> {
        Self::try_from(val).map_err(|e| match e {
            CreatePriceError::InvalidScale => error_list.lock().unwrap().push(
                ValidationError::new("Price scale must not be > 2".to_string()),
            ),
            CreatePriceError::NegativeValue => {}
        })
    }
}
