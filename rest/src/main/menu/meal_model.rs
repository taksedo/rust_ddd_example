use bigdecimal::BigDecimal;
use serde::Deserialize;
use serde_derive::Serialize;
use usecase::main::menu::dto::meal_info::MealInfo;
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct MealModel {
    /// ID of the meal
    pub id: i64,
    /// Name of the meal
    pub name: String,
    /// Description of the meal
    pub description: String,
    /// Price of the meal
    #[schema(value_type = f64)]
    pub price: BigDecimal,
    /// Version of the meal
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
