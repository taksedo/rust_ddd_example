use common_types::main::base::domain_entity::{DomainEntity, Version};

use crate::main::menu::meal_events::DomainEventEnum;
use crate::main::menu::value_objects::meal_description::MealDescription;
use crate::main::menu::value_objects::meal_id::MealId;
use crate::main::menu::value_objects::price::Price;

use super::meal::Meal;
use super::value_objects::meal_name::MealName;

pub struct MealRestorer {}

impl MealRestorer {
    pub fn restore_meal(
        id: MealId,
        name: MealName,
        description: MealDescription,
        price: Price,
        removed: bool,
        version: Version,
        events: Vec<DomainEventEnum>,
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
