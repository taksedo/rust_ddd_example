use super::meal::Meal;
use super::meal_id::MealId;
use super::meal_name::MealName;
use common_types::main::base::domain_entity::{DomainEntity, Version};
use common_types::main::base::domain_event::DomainEventTrait;
use std::cell::RefCell;
use std::rc::Rc;

pub struct MealRestorer {}

impl MealRestorer {
    pub fn restore_meal(
        id: MealId,
        name: MealName,
        removed: bool,
        version: Version,
        events: Vec<Rc<RefCell<dyn DomainEventTrait>>>,
    ) -> Meal {
        Meal {
            domain_entity_field: DomainEntity {
                id,
                version,
                events,
            },
            name,
            removed,
        }
    }
}
