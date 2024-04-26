use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use common::types::{
    base::domain_entity::{DomainEntity, DomainEntityTrait, Version},
    errors::error::BusinessError,
};
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::main::menu::{
    meal_already_exists::MealAlreadyExists,
    meal_events::{MealAddedToMenuDomainEvent, MealEventEnum, MealRemovedFromMenuDomainEvent},
    value_objects::{
        meal_description::MealDescription,
        meal_id::{MealId, MealIdGenerator},
        meal_name::MealName,
        price::Price,
    },
};

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
            let id = self.entity_params.id;
            self.entity_params
                .add_event(MealRemovedFromMenuDomainEvent::new(id).into())
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MealError {
    AlreadyExistsWithSameNameError,
}

impl BusinessError for MealError {}
