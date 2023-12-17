#![allow(non_snake_case)]

use rstest::rstest;

use crate::main::menu::value_objects::meal_name::{CreateMealNameError, MealName};

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
