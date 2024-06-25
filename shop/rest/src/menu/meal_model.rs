use bigdecimal::BigDecimal;
use serde::Deserialize;
use serde_derive::Serialize;
use usecase::menu::dto::meal_info::MealInfo;
use utoipa::{ToResponse, ToSchema};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema, ToResponse)]
pub struct MealModel {
    /// ID of the meal
    #[schema(example = 1)]
    pub id: i64,
    /// Name of the meal
    #[schema(example = "Шаурма маленькая")]
    pub name: String,
    /// Description of the meal
    #[schema(example = "Хоть маленькая, но всё такая же вкусная")]
    pub description: String,
    /// Price of the meal
    #[schema(value_type = String, example = "149.99")]
    pub price: BigDecimal,
    /// Version of the meal
    #[schema(example = 1)]
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
