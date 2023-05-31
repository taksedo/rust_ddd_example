use serde::Deserialize;
use serde_derive::Serialize;
use usecase::main::menu::dto::meal_info::MealInfo;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MealModel {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub version: u64,
}

impl MealModel {
    pub fn from(meal_info: MealInfo) -> Self {
        Self {
            id: meal_info.id.to_u64(),
            name: meal_info.name.to_string(),
            description: meal_info.description.to_string(),
            price: meal_info.price.to_f64(),
            version: meal_info.version.to_u64(),
        }
    }
}
