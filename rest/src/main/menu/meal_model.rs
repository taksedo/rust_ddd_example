use bigdecimal::BigDecimal;
use serde::Deserialize;
use serde_derive::Serialize;

use usecase::main::menu::dto::meal_info::MealInfo;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MealModel {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub price: BigDecimal,
    pub version: i64,
}

impl MealModel {
    pub fn from(meal_info: MealInfo) -> Self {
        Self {
            id: meal_info.id.to_i64(),
            name: meal_info.name.to_string(),
            description: meal_info.description.to_string(),
            price: meal_info.price.to_bigdecimal(),
            version: meal_info.version.to_i64(),
        }
    }
}
