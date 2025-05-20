use std::fmt::Debug;

use async_trait::async_trait;

use crate::menu::value_objects::{meal_id::MealId, price::Price};

#[async_trait]
pub trait GetMealPrice: Debug + Send {
    async fn invoke(&self, for_meal_id: &MealId) -> Price;
}
