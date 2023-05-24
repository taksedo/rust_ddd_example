use domain::main::menu::meal_name::{CreateMealNameError, MealName};

pub trait Validated<V> {
    fn validated(val: String) -> Result<V, CreateMealNameError>;
}

impl Validated<MealName> for MealName {
    fn validated(val: String) -> Result<MealName, CreateMealNameError> {
        MealName::from(val)
    }
}
