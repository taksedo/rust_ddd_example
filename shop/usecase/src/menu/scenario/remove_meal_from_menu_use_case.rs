use std::sync::{Arc, Mutex};

use derive_new::new;
use domain::menu::value_objects::meal_id::MealId;

use crate::menu::{
    access::{meal_extractor::MealExtractor, meal_persister::MealPersister},
    remove_meal_from_menu::{RemoveMealFromMenu, RemoveMealFromMenuUseCaseError},
};

#[derive(Debug, new)]
pub struct RemoveMealFromMenuUseCase {
    pub meal_extractor: Arc<Mutex<dyn MealExtractor>>,
    pub meal_persister: Arc<Mutex<dyn MealPersister>>,
}

impl RemoveMealFromMenu for RemoveMealFromMenuUseCase {
    fn execute(&mut self, id: &MealId) -> Result<(), RemoveMealFromMenuUseCaseError> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain_test_fixtures::{rnd_meal, rnd_meal_id},
        test_fixtures::{MockMealExtractor, MockMealPersister},
    };

    #[test]
    fn successfully_removed() {
        let meal = rnd_meal();

        let meal_persister = Arc::new(Mutex::new(MockMealPersister::new()));
        let meal_extractor = Arc::new(Mutex::new(MockMealExtractor::new()));
        meal_extractor.lock().unwrap().meal = Some(meal.clone());

        let mut use_case =
            RemoveMealFromMenuUseCase::new(meal_extractor.clone(), meal_persister.clone());
        let result = use_case.execute(meal.id());

        assert!(result.is_ok());

        let meal = meal_persister.lock().unwrap().meal.clone().unwrap();
        //todo: придумать более изящное тестирование meal

        meal_persister.lock().unwrap().verify_invoked_meal(&meal);

        meal_extractor
            .lock()
            .unwrap()
            .verify_invoked_get_by_id(&meal.id());

        meal_persister
            .lock()
            .unwrap()
            .verify_events_after_deletion(&meal.id());
    }

    #[test]
    fn meal_not_found() {
        let meal_persister = Arc::new(Mutex::new(MockMealPersister::new()));
        let meal_extractor = Arc::new(Mutex::new(MockMealExtractor::new()));
        let mut use_case =
            RemoveMealFromMenuUseCase::new(meal_extractor.clone(), meal_persister.clone());

        let meal_id = rnd_meal_id();

        let result = use_case.execute(&meal_id);

        assert_eq!(result, Err(RemoveMealFromMenuUseCaseError::MealNotFound));
        meal_persister.lock().unwrap().verify_empty();

        meal_extractor
            .lock()
            .unwrap()
            .verify_invoked_get_by_id(&meal_id);
    }
}
