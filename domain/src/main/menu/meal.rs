use crate::main::menu::meal_already_exists::MealAlreadyExists;
use crate::main::menu::meal_events::{
    DomainEventEnum, MealAddedToMenuDomainEvent, MealRemovedFromMenuDomainEvent,
};
use crate::main::menu::meal_id::{MealId, MealIdGenerator};
use crate::main::menu::meal_name::MealName;
use common_types::main::base::domain_entity::{DomainEntity, DomainEntityTrait, Version};
use common_types::main::errors::error::BusinessError;
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(new, Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Meal {
    pub domain_entity_field: DomainEntity<MealId, DomainEventEnum>,
    pub name: MealName,
    #[new(value = "false")]
    pub removed: bool,
}

impl Meal {
    pub fn add_meal_to_menu(
        id_generator: Rc<RefCell<dyn MealIdGenerator>>,
        meal_exists: Rc<RefCell<dyn MealAlreadyExists>>,
        name: MealName,
    ) -> Result<Meal, MealError> {
        if meal_exists.borrow_mut().invoke(&name) {
            Err(MealError::AlreadyExistsWithSameNameError)
        } else {
            let id = id_generator.borrow().generate();

            //     .map_err(|_e: Error| MealError::IdGenerationError)?;
            let mut meal = Meal::new(DomainEntity::new(id, Version::new()), name);
            meal.add_event(DomainEventEnum::MealAddedToMenuDomainEvent(
                MealAddedToMenuDomainEvent::new(id),
            ));
            Ok(meal)
        }
    }

    pub fn visible(&self) -> bool {
        !self.removed
    }

    pub fn remove_meal_from_menu(&mut self) {
        if !self.removed {
            self.removed = true;
            let removing_event = MealRemovedFromMenuDomainEvent::new(self.domain_entity_field.id);
            self.add_event(DomainEventEnum::MealRemovedFromMenuDomainEvent(
                removing_event,
            ))
        }
    }
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum MealError {
    #[error("Еда с таким именем уже существует")]
    AlreadyExistsWithSameNameError,
    #[error("Ошибка при генерации id")]
    IdGenerationError,
}

impl DomainEntityTrait<DomainEventEnum> for Meal {
    fn add_event(&mut self, event: DomainEventEnum) {
        if self.domain_entity_field.events.is_empty() {}
        self.domain_entity_field.events.push(event)
    }
    fn pop_events(&self) -> &Vec<DomainEventEnum> {
        &self.domain_entity_field.events
    }
}

impl BusinessError for MealError {}
