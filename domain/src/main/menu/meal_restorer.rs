use super::meal::Meal;
use super::meal_id::MealId;
use super::meal_name::MealName;
use crate::main::menu::meal_description::MealDescription;
use crate::main::menu::meal_events::DomainEventEnum;
use crate::main::menu::price::Price;
use common_types::main::base::domain_entity::{DomainEntity, Version};

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
            domain_entity_field: DomainEntity {
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
