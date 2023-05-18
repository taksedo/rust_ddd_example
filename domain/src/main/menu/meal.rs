use crate::main::menu::meal_already_exists::MealAlreadyExists;
use crate::main::menu::meal_events::{MealAddedToMenuDomainEvent, MealRemovedFromMenuDomainEvent};
use crate::main::menu::meal_id::{MealId, MealIdGenerator};
use crate::main::menu::meal_name::MealName;
use common_types::main::base::domain_entity::{DomainEntity, DomainEntityTrait, Version};
use common_types::main::base::domain_event::DomainEventTrait;
use common_types::main::errors::error::BusinessError;
use derive_new::new;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(new, Debug, Clone, PartialEq)]
pub struct Meal {
    pub domain_entity_field: DomainEntity<MealId>,
    pub name: MealName,
    #[new(value = "false")]
    pub removed: bool,
}

impl Meal {
    pub fn add_meal_to_menu(
        id_generator: Rc<dyn MealIdGenerator>,
        meal_exists: Rc<RefCell<dyn MealAlreadyExists>>,
        name: MealName,
    ) -> Result<Meal, MealError> {
        if meal_exists.borrow_mut().invoke(&name) {
            Err(MealError::AlreadyExistsWithSameNameError)
        } else {
            let id = id_generator.generate();

            //     .map_err(|_e: Error| MealError::IdGenerationError)?;
            let mut meal = Meal::new(DomainEntity::new(id, Version::new()), name);
            meal.add_event(Rc::new(RefCell::new(MealAddedToMenuDomainEvent::new(id))));
            Ok(meal)
        }
    }

    pub fn visible(&self) -> bool {
        !self.removed
    }

    pub fn remove_meal_from_menu(&mut self) {
        if !self.removed {
            self.removed = true;
            let removing_event = Rc::new(RefCell::new(MealRemovedFromMenuDomainEvent::new(
                self.domain_entity_field.id,
            )));
            self.add_event(removing_event)
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

impl DomainEntityTrait for Meal {
    fn add_event(&mut self, event: Rc<RefCell<dyn DomainEventTrait>>) {
        if self.domain_entity_field.events.is_empty() {}
        self.domain_entity_field.events.push(event)
    }
    fn pop_events(&self) -> &Vec<Rc<RefCell<dyn DomainEventTrait>>> {
        &self.domain_entity_field.events
    }
}

impl BusinessError for MealError {}
