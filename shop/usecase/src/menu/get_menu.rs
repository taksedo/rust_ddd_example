use async_trait::async_trait;

use crate::menu::dto::meal_info::MealInfo;

#[async_trait]
pub trait GetMenu {
    async fn execute(&self) -> Vec<MealInfo>;
}
