#![allow(non_snake_case)]

use crate::main::menu::value_objects::meal_description::{
    CreateMealDescriptionError, MealDescription,
};
use mockall::Any;
use rstest::rstest;

#[test]
fn create_description__success() {
    let value = "Some string";
    let result = MealDescription::try_from(value).unwrap();

    let type_name = result.type_name();
    assert_eq!(
        type_name,
        "domain::main::menu::value_objects::meal_description::MealDescription"
    );
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
