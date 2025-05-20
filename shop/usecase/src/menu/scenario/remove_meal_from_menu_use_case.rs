use async_trait::async_trait;
use common::types::base::AM;
use derive_new::new;
use domain::menu::value_objects::meal_id::MealId;

use crate::menu::{
    access::{meal_extractor::MealExtractor, meal_persister::MealPersister},
    remove_meal_from_menu::{RemoveMealFromMenu, RemoveMealFromMenuUseCaseError},
};

#[derive(Debug, new)]
pub struct RemoveMealFromMenuUseCase {
    pub meal_extractor: AM<dyn MealExtractor>,
    pub meal_persister: AM<dyn MealPersister>,
}

#[async_trait]
impl RemoveMealFromMenu for RemoveMealFromMenuUseCase {
    async fn execute(&mut self, id: &MealId) -> Result<(), RemoveMealFromMenuUseCaseError> {
        let mut meal = self
            .meal_extractor
            .lock()
            .await
            .get_by_id(id)
            .ok_or(RemoveMealFromMenuUseCaseError::MealNotFound)?;
        meal.remove_meal_from_menu();
        self.meal_persister.lock().await.save(meal).await;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use common::types::base::AMTrait;
    use domain::test_fixtures::*;

    use super::*;
    use crate::test_fixtures::{MockMealExtractor, MockMealPersister};

    #[tokio::test]
    async fn successfully_removed() {
        let meal = rnd_meal();

        let meal_persister = AM::new_am(MockMealPersister::new());
        let meal_extractor = AM::new_am(MockMealExtractor::new());
        meal_extractor.lock().await.meal = Some(meal.clone());

        let mut use_case =
            RemoveMealFromMenuUseCase::new(meal_extractor.clone(), meal_persister.clone());
        let result = use_case.execute(meal.id()).await;

        assert!(result.is_ok());

        let meal = meal_persister.lock().await.meal.clone().unwrap();
        //todo: придумать более изящное тестирование meal

        meal_persister.lock().await.verify_invoked_meal(&meal);

        meal_extractor
            .lock()
            .await
            .verify_invoked_get_by_id(meal.id());

        meal_persister
            .lock()
            .await
            .verify_events_after_deletion(meal.id());
    }

    #[tokio::test]
    async fn meal_not_found() {
        let meal_persister = AM::new_am(MockMealPersister::new());
        let meal_extractor = AM::new_am(MockMealExtractor::new());
        let mut use_case =
            RemoveMealFromMenuUseCase::new(meal_extractor.clone(), meal_persister.clone());

        let meal_id = rnd_meal_id();

        let result = use_case.execute(&meal_id).await;

        assert_eq!(result, Err(RemoveMealFromMenuUseCaseError::MealNotFound));
        meal_persister.lock().await.verify_empty();

        meal_extractor
            .lock()
            .await
            .verify_invoked_get_by_id(&meal_id);
    }
}
