use async_trait::async_trait;
use common::types::base::AM;
use derive_new::new;

use crate::menu::{
    access::meal_extractor::MealExtractor, dto::meal_info::MealInfo, get_menu::GetMenu,
};

#[derive(Debug, new)]
pub struct GetMenuUseCase {
    pub(crate) meal_extractor: AM<dyn MealExtractor>,
}

#[async_trait]
impl GetMenu for GetMenuUseCase {
    async fn execute(&self) -> Vec<MealInfo> {
        self.meal_extractor
            .lock()
            .await
            .get_all()
            .into_iter()
            .map(MealInfo::from)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use common::types::base::AMTrait;
    use domain::test_fixtures::*;
    use tokio::test;

    use super::*;
    use crate::test_fixtures::MockMealExtractor;

    #[test]
    #[allow(non_snake_case)]
    async fn get_menu__menu_is_empty() {
        let meal_extractor = MockMealExtractor::new();
        let use_case = GetMenuUseCase::new(AM::new_am(meal_extractor));
        let menu = use_case.execute().await;

        assert!(menu.is_empty());
        use_case
            .meal_extractor
            .lock()
            .await
            .downcast_ref::<MockMealExtractor>()
            .unwrap()
            .verify_invoked_get_all();
    }

    #[test]
    async fn get_menu() {
        let meal = rnd_meal();
        let meal_extractor = MockMealExtractor {
            meal: Option::from(meal.to_owned()),
            ..MockMealExtractor::default()
        };
        let use_case = GetMenuUseCase::new(AM::new_am(meal_extractor));
        let menu = use_case.execute().await;

        assert_eq!(
            menu,
            vec![MealInfo {
                id: *meal.id(),
                name: meal.name().to_owned(),
                description: meal.description().to_owned(),
                price: meal.price().to_owned(),
                version: *meal.version(),
            }]
        );
        use_case
            .meal_extractor
            .lock()
            .await
            .downcast_ref::<MockMealExtractor>()
            .unwrap()
            .verify_invoked_get_all();
    }
}
