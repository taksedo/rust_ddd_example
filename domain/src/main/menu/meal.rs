use crate::main::menu::meal_id::{MealId, MealIdGenerator};
use crate::main::menu::meal_name::MealName;
use common::main::types::errors::error::BusinessError;
use derive_new::new;
use std::rc::Rc;

#[derive(new, Debug, Clone, PartialEq)]
pub struct Meal {
    pub id: MealId,
    pub name: MealName,
}

impl Meal {
    pub fn add_meal_to_menu(
        id_generator: Rc<dyn MealIdGenerator>,
        name: MealName,
    ) -> Result<Meal, MealError> {
        let id = id_generator.generate();
        Ok(Self { id, name })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum MealError {
    #[error("Еда с таким именем уже существует")]
    AlreadyExistsWithSameNameError,
}

impl BusinessError for MealError {}
