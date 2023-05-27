#![allow(non_snake_case)]

use crate::main::menu::meal_description::{CreateMealDescriptionError, MealDescription};
use mockall::Any;
use rstest::rstest;

#[test]
fn create_description__success() {
    let value = "Some string".to_string();
    let result = MealDescription::new(value.to_owned());

    let type_name = result.type_name();
    assert_eq!(
        type_name,
        "domain::main::menu::meal_description::MealDescription"
    );
    let description = result;
    assert_eq!(description.to_string_value(), value);
}

#[rstest]
#[case("")]
#[case(" ")]
fn create_description__empty_string(#[case] input: String) {
    let result = MealDescription::from(input);
    assert_eq!(
        result,
        Err(CreateMealDescriptionError::EmptyDescriptionError)
    );
}
