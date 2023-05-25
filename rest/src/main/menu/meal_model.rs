use usecase::main::menu::dto::meal_info::MealInfo;

#[derive(Debug, Clone, PartialEq)]
pub struct MealModel {
    pub id: u64,
    pub name: String,
    pub version: u64,
}

impl MealModel {
    pub fn from(meal_info: MealInfo) -> Self {
        Self {
            id: meal_info.id.to_u64().clone(),
            name: meal_info.name.to_string_value().clone(),
            version: meal_info.version.to_u64().clone(),
        }
    }
}
