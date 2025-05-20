use async_trait::async_trait;
use common::types::base::AM;
use derive_new::new;
use domain::{
    menu::value_objects::{meal_id::MealId, price::Price},
    order::get_meal_price::GetMealPrice,
};

use crate::menu::access::meal_extractor::MealExtractor;

#[async_trait]
impl GetMealPrice for GetMealPriceUsingExtractor {
    async fn invoke(&self, for_meal_id: &MealId) -> Price {
        let meal = &self.extractor.lock().await.get_by_id(for_meal_id);
        assert!(meal.is_some(), "Meal #{:?} not found", for_meal_id);
        meal.clone().unwrap().price().clone()
    }
}

#[derive(new, Debug)]
pub struct GetMealPriceUsingExtractor {
    pub extractor: AM<dyn MealExtractor>,
}

#[cfg(test)]
mod tests {
    use common::types::base::AMTrait;
    use domain::test_fixtures::*;

    use super::*;
    use crate::test_fixtures::MockMealExtractor;

    #[tokio::test]
    async fn price_has_been_provided() {
        let meal = rnd_meal();

        let extractor = AM::new_am(MockMealExtractor::new());
        extractor.lock().await.meal = Some(meal.clone());

        let get_meal_price = GetMealPriceUsingExtractor::new(extractor.clone());
        let result = get_meal_price.invoke(meal.id()).await;

        extractor.lock().await.verify_invoked_get_by_id(meal.id());
        assert_eq!(result, meal.price().to_owned());
    }

    #[test]
    fn meal_not_found() {
        use std::panic::AssertUnwindSafe;

        use assert_panic::assert_panic;

        let meal_id = rnd_meal_id();
        let extractor = AM::new_am(MockMealExtractor::new());
        let usecase = GetMealPriceUsingExtractor::new(extractor.clone());

        let safe_usecase = AssertUnwindSafe(&usecase);

        assert_panic!({
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async {
                safe_usecase.invoke(&meal_id).await;
            })
        },
        String,
        starts with &format!("Meal #{:?} not found", meal_id));

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                extractor.lock().await.verify_invoked_get_by_id(&meal_id);
            });
        // extractor.lock().await.verify_invoked_get_by_id(&meal_id);
    }
}
