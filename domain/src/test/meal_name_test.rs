#![allow(non_snake_case)]
use crate::main::menu::meal_name::{CreateMealNameError, MealName};
use rstest::rstest;

#[test]
fn create_name__success() {
    let value = "Some string".to_string();
    let result = MealName::from(value.clone());

    let meal_name = result.unwrap();
    assert_eq!(meal_name.to_string(), value);
}

#[rstest]
#[case("")]
#[case(" ")]
fn create_name__empty_string(#[case] input: String) {
    let result = MealName::from(input);
    assert_eq!(result, Err(CreateMealNameError::EmptyMealNameError));
}
