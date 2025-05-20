use std::fmt::Debug;

use crate::menu::value_objects::meal_name::MealName;

#[async_trait::async_trait]
pub trait MealAlreadyExists: Debug + Send {
    async fn invoke(&mut self, name: &MealName) -> bool;
}
