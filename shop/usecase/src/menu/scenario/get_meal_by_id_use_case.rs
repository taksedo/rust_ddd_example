use async_trait::async_trait;
use common::types::base::AM;
use derive_new::new;
use domain::menu::value_objects::meal_id::MealId;

use crate::menu::{
    access::meal_extractor::MealExtractor,
    dto::meal_info::MealInfo,
    get_meal_by_id::{GetMealById, GetMealByIdUseCaseError},
};

#[derive(new, Debug)]
pub struct GetMealByIdUseCase {
    pub meal_extractor: AM<dyn MealExtractor>,
}

#[async_trait]
impl GetMealById for GetMealByIdUseCase {
    async fn execute(&mut self, id: &MealId) -> Result<MealInfo, GetMealByIdUseCaseError> {
        match self.meal_extractor.lock().await.get_by_id(id) {
            res if res.is_some() && res.clone().unwrap().visible() => {
                let res = res.unwrap();
                Ok(MealInfo::from(res))
            }
            _ => Err(GetMealByIdUseCaseError::MealNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use common::types::base::AMTrait;
    use domain::test_fixtures::*;
    use tokio::test;

    use super::*;
    use crate::test_fixtures::{MockMealExtractor, removed_meal};

    #[test]
    async fn meal_not_found() {
        let meal_extractor = AM::new_am(MockMealExtractor::new());
        let mut use_case = GetMealByIdUseCase::new(meal_extractor);

        let meal_id = &rnd_meal_id();
        let result = use_case.execute(meal_id).await;

        assert_eq!(result, Err(GetMealByIdUseCaseError::MealNotFound));
        use_case
            .meal_extractor
            .lock()
            .await
            .downcast_ref::<MockMealExtractor>()
            .unwrap()
            .verify_invoked_get_by_id(meal_id);
    }

    #[test]
    async fn meal_removed() {
        let meal = removed_meal();
        let meal_extractor = AM::new_am(MockMealExtractor {
            meal: Option::from(meal.to_owned()),
            ..MockMealExtractor::default()
        });

        let mut use_case = GetMealByIdUseCase::new(meal_extractor);
        let result = use_case.execute(meal.id()).await;

        assert_eq!(result, Err(GetMealByIdUseCaseError::MealNotFound));
        use_case
            .meal_extractor
            .lock()
            .await
            .downcast_ref::<MockMealExtractor>()
            .unwrap()
            .verify_invoked_get_by_id(meal.id());
    }

    #[test]
    async fn meal_extracted_successfully() {
        let meal = rnd_meal();
        let meal_extractor = AM::new_am(MockMealExtractor {
            meal: Option::from(meal.to_owned()),
            ..MockMealExtractor::default()
        });
        let mut use_case = GetMealByIdUseCase::new(meal_extractor);

        let result = use_case.execute(meal.id()).await;
        let meal_info = result;

        assert_eq!(
            meal_info.unwrap(),
            MealInfo {
                id: *meal.id(),
                name: meal.name().to_owned(),
                description: meal.description().to_owned(),
                price: meal.price().to_owned(),
                version: *meal.version(),
            }
        );
        use_case
            .meal_extractor
            .lock()
            .await
            .downcast_ref::<MockMealExtractor>()
            .unwrap()
            .verify_invoked_get_by_id(meal.id());
    }
}
