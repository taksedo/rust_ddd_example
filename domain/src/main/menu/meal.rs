use crate::main::menu::meal_events::MealRemovedFromMenuDomainEvent;
use crate::main::menu::meal_id::{MealId, MealIdGenerator};
use crate::main::menu::meal_name::MealName;
use common_types::main::base::domain_entity::{DomainEntityTrait, Version};
use common_types::main::base::domain_event::DomainEventTrait;
use common_types::main::errors::error::BusinessError;
use derivative::Derivative;
use derive_new::new;
use std::fmt::Error;

#[derive(new, Debug, Derivative, Clone)]
#[derivative(PartialEq)]
pub struct Meal<E: DomainEventTrait + Clone> {
    pub id: MealId,
    pub name: MealName,
    version: Version,
    #[new(value = "false")]
    pub removed: bool,

    #[derivative(PartialEq = "ignore")]
    #[new(value = "vec![] as Vec<E>")]
    pub events: Vec<E>,
}

impl<E: DomainEventTrait + Clone> Meal<E> {
    pub fn add_meal_to_menu<I: MealIdGenerator>(
        id_generator: &I,
        name: MealName,
    ) -> Result<Meal<E>, MealError> {
        Ok(id_generator.generate())
            .map_err(|_e: Error| MealError::IdGenerationError)
            .map(|id| Meal::new(id, name, Version::new()))
    }

    pub fn visible(&self) -> bool {
        !self.removed
    }
}

impl Meal<MealRemovedFromMenuDomainEvent> {
    pub fn remove_meal_from_menu(&mut self) {
        if !self.removed {
            self.removed = true;
            let removing_event = MealRemovedFromMenuDomainEvent::new(self.id);
            self.add_event(removing_event)
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum MealError {
    #[error("Еда с таким именем уже существует")]
    AlreadyExistsWithSameNameError,
    #[error("Ошибка при генерации id")]
    IdGenerationError,
}

impl<E: DomainEventTrait + Clone> DomainEntityTrait<E> for Meal<E> {
    fn add_event(&mut self, event: E) {
        if self.events.is_empty() {}
        self.events.push(event)
    }
    fn pop_events(&self) -> &Vec<E> {
        &self.events
    }
}

impl BusinessError for MealError {}
