use std::sync::{Arc, Mutex};

use derive_new::new;
use domain::main::menu::value_objects::meal_id::MealId;

use crate::main::menu::{
    access::{meal_extractor::MealExtractor, meal_persister::MealPersister},
    remove_meal_from_menu::{RemoveMealFromMenu, RemoveMealFromMenuUseCaseError},
};

#[derive(Debug, new)]
pub struct RemoveMealFromMenuUseCase {
    pub meal_extractor: Arc<Mutex<dyn MealExtractor>>,
    pub meal_persister: Arc<Mutex<dyn MealPersister>>,
}

impl RemoveMealFromMenu for RemoveMealFromMenuUseCase {
    fn execute(&mut self, id: MealId) -> Result<(), RemoveMealFromMenuUseCaseError> {
        let mut meal = self
            .meal_extractor
            .lock()
            .unwrap()
            .get_by_id(id)
            .map_or(Err(RemoveMealFromMenuUseCaseError::MealNotFound), |meal| {
                Ok(meal)
            })?;
        meal.remove_meal_from_menu();
        self.meal_persister.lock().unwrap().save(meal);
        Ok(())
    }
}
