use bigdecimal::*;
use domain::main::menu::value_objects::meal_description::{
    CreateMealDescriptionError, MealDescription,
};
use domain::main::menu::value_objects::meal_name::{CreateMealNameError, MealName};
use domain::main::menu::value_objects::price::{CreatePriceError, Price};

pub trait Validated<V, R, S> {
    fn validated(val: S) -> Result<V, R>;
}

impl Validated<MealName, CreateMealNameError, String> for MealName {
    fn validated(val: String) -> Result<MealName, CreateMealNameError> {
        Self::from(val)
    }
}

impl Validated<MealDescription, CreateMealDescriptionError, String> for MealDescription {
    fn validated(val: String) -> Result<MealDescription, CreateMealDescriptionError> {
        Self::from(val)
    }
}

impl Validated<Price, CreatePriceError, BigDecimal> for Price {
    fn validated(val: BigDecimal) -> Result<Price, CreatePriceError> {
        Self::from(val)
    }
}
