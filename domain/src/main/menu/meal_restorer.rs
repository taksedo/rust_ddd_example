use common::types::main::base::domain_entity::{DomainEntity, Version};

use super::{meal::Meal, value_objects::meal_name::MealName};
use crate::main::menu::{
    meal_events::MealEventEnum,
    value_objects::{meal_description::MealDescription, meal_id::MealId, price::Price},
};

pub struct MealRestorer {}

impl MealRestorer {
    pub fn restore_meal(
        id: MealId,
        name: MealName,
        description: MealDescription,
        price: Price,
        removed: bool,
        version: Version,
        events: Vec<MealEventEnum>,
    ) -> Meal {
        Meal {
            entity_params: DomainEntity {
                id,
                version,
                events,
            },
            name,
            description,
            price,
            removed,
        }
    }
}
