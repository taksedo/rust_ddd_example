use async_trait::async_trait;
use common::types::base::AM;
use derive_new::new;
use domain::menu::{meal_already_exists::MealAlreadyExists, value_objects::meal_name::MealName};

use crate::menu::access::meal_extractor::MealExtractor;

#[derive(new, Debug, Clone)]
pub struct MealAlreadyExistsUsesMealExtractor {
    pub extractor: AM<dyn MealExtractor>,
}

#[async_trait]
impl MealAlreadyExists for MealAlreadyExistsUsesMealExtractor {
    async fn invoke(&mut self, name: &MealName) -> bool {
        let meal_found_by_get = self.extractor.lock().await.get_by_name(name);
        let meal_found_by_get_is_removed = *meal_found_by_get.clone().unwrap_or_default().removed();
        meal_found_by_get.is_some() & !meal_found_by_get_is_removed
    }
}

#[cfg(test)]
mod tests {
    use common::types::base::AMTrait;
    use domain::test_fixtures::{rnd_meal, rnd_meal_name};
    use tokio::test;

    use super::*;
    use crate::test_fixtures::{MockMealExtractor, removed_meal};

    #[test]
    async fn meal_already_exists() {
        let meal = rnd_meal();
        let extractor = AM::new_am(MockMealExtractor {
            meal: Some(meal.to_owned()),
            ..MockMealExtractor::default()
        });
        let mut rule = MealAlreadyExistsUsesMealExtractor::new(extractor);

        let result = rule.invoke(meal.name()).await;

        assert!(result);

        rule.extractor
            .lock()
            .await
            .downcast_ref::<MockMealExtractor>()
            .unwrap()
            .verify_invoked_get_by_name(meal.name());
    }

    #[test]
    async fn meal_already_exists_but_removed() {
        let meal = removed_meal();
        let extractor = AM::new_am(MockMealExtractor {
            meal: Some(meal.to_owned()),
            ..MockMealExtractor::default()
        });
        let mut rule = MealAlreadyExistsUsesMealExtractor::new(extractor);

        let result = rule.invoke(meal.name()).await;

        assert!(!result);
        rule.extractor
            .lock()
            .await
            .downcast_ref::<MockMealExtractor>()
            .unwrap()
            .verify_invoked_get_by_name(meal.name());
    }

    #[test]
    async fn meal_already_exists_doesnt_exist() {
        let extractor = AM::new_am(MockMealExtractor::new());
        let mut rule = MealAlreadyExistsUsesMealExtractor::new(extractor);

        let meal_name = rnd_meal_name();
        let result = rule.invoke(&meal_name).await;

        assert!(!result);
        rule.extractor
            .lock()
            .await
            .downcast_ref::<MockMealExtractor>()
            .unwrap()
            .verify_invoked_get_by_name(&meal_name);
    }
}
