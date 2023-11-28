use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use common::types::main::base::domain_entity::{DomainEntity, DomainEntityTrait, Version};
use common::types::main::errors::error::BusinessError;
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::main::menu::meal_already_exists::MealAlreadyExists;
use crate::main::menu::meal_events::{
    MealAddedToMenuDomainEvent, MealEventEnum, MealRemovedFromMenuDomainEvent,
};
use crate::main::menu::value_objects::meal_description::MealDescription;
use crate::main::menu::value_objects::meal_id::{MealId, MealIdGenerator};
use crate::main::menu::value_objects::meal_name::MealName;
use crate::main::menu::value_objects::price::Price;

#[derive(new, Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Meal {
    pub entity_params: DomainEntity<MealId, MealEventEnum>,
    pub name: MealName,
    pub description: MealDescription,
    pub price: Price,
    #[new(value = "false")]
    pub removed: bool,
}

impl Meal {
    pub fn add_meal_to_menu(
        id_generator: Arc<Mutex<dyn MealIdGenerator>>,
        meal_exists: Arc<Mutex<dyn MealAlreadyExists>>,
        name: MealName,
        description: MealDescription,
        price: Price,
    ) -> Result<Meal, MealError> {
        if meal_exists.lock().unwrap().invoke(&name) {
            Err(MealError::AlreadyExistsWithSameNameError)
        } else {
            let id = id_generator.lock().unwrap().generate();

            //     .map_err(|_e: Error| MealError::IdGenerationError)?;
            let mut meal = Meal::new(
                DomainEntity::new(id, Version::default()),
                name,
                description,
                price,
            );
            meal.entity_params
                .add_event(MealAddedToMenuDomainEvent::new(id).into());
            Ok(meal)
        }
    }

    pub fn visible(&self) -> bool {
        !self.removed
    }

    pub fn remove_meal_from_menu(&mut self) {
        if !self.removed {
            self.removed = true;
            let removing_event = MealRemovedFromMenuDomainEvent::new(self.entity_params.id);
            self.entity_params.add_event(removing_event.into())
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MealError {
    AlreadyExistsWithSameNameError,
    IdGenerationError,
}

impl BusinessError for MealError {}
