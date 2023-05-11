use crate::main::menu::add_meal_to_menu::{AddMealToMenuRequest, InvalidMealParametersError};
use domain::test_fixtures::fixtures::rnd_meal_name;

#[test]
fn successfully_created() {
    let name = rnd_meal_name();
    let result = AddMealToMenuRequest::from(name.to_owned().value).unwrap();
    assert_eq!(result, AddMealToMenuRequest::new(name))
}

#[test]
fn invalid_name() {
    let name = "".to_string();

    let result = AddMealToMenuRequest::from(name);

    assert_eq!(
        result.unwrap_err(),
        InvalidMealParametersError::InvalidParameters
    )
}
