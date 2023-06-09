use crate::main::menu::access::meal_extractor::MealExtractor;
use crate::main::menu::access::meal_persister::MealPersister;
use crate::main::menu::remove_meal_from_menu::{
    RemoveMealFromMenu, RemoveMealFromMenuUseCaseError,
};
use derive_new::new;
use domain::main::menu::meal_id::MealId;
use std::sync::{Arc, Mutex};

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
