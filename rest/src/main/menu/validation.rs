use crate::main::menu::add_meal_to_meny_endpoint::AddMealToMenuRestRequest;
use common_rest::main::global_error_handler::RestHttpError;
use domain::main::menu::meal_name::{CreateMealNameError, MealName};
use std::error::Error;

trait Validated<V> {
    fn validated(val: String) -> Result<V, impl Error>;
}

impl Validated<MealName> for AddMealToMenuRestRequest {
    fn validated(val: String) -> Result<MealName, CreateMealNameError> {
        MealName::from(val)
    }
}
