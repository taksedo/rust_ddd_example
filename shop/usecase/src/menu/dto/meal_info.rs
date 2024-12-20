use common::types::base::Version;
use domain::menu::{
    meal::Meal,
    value_objects::{
        meal_description::MealDescription, meal_id::MealId, meal_name::MealName, price::Price,
    },
};

/// На данный момент эта dto используется в нескольких сценариях.
/// Тут следует быть осторожным и вовремя заметить, когда разным сценариям нужен будет разный набор данных
/// Если такое произойдёт — необходимо выделить отдельный класс. В нашем случае можно оставить и так.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct MealInfo {
    pub id: MealId,
    pub name: MealName,
    pub description: MealDescription,
    pub price: Price,
    pub version: Version,
}

impl From<Meal> for MealInfo {
    fn from(value: Meal) -> Self {
        Self {
            id: *value.id(),
            name: value.name().to_owned(),
            description: value.description().to_owned(),
            price: value.price().to_owned(),
            version: *value.version(),
        }
    }
}
