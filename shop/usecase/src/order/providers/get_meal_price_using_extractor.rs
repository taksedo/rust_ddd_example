use common::types::base::{AM, AMTrait};
use derive_new::new;
use domain::{
    menu::value_objects::{meal_id::MealId, price::Price},
    order::get_meal_price::GetMealPrice,
};

use crate::menu::access::meal_extractor::MealExtractor;

impl GetMealPrice for GetMealPriceUsingExtractor {
    fn invoke(&self, for_meal_id: &MealId) -> Price {
        let meal = &self.extractor.lock_un().get_by_id(for_meal_id);
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
    use assert_panic::assert_panic;
    use domain::test_fixtures::*;

    use super::*;
    use crate::test_fixtures::MockMealExtractor;

    #[test]
    fn price_has_been_provided() {
        let meal = rnd_meal();

        let extractor = AM::new_am(MockMealExtractor::new());
        extractor.lock_un().meal = Some(meal.clone());

        let get_meal_price = GetMealPriceUsingExtractor::new(extractor.clone());
        let result = get_meal_price.invoke(meal.id());

        extractor.lock_un().verify_invoked_get_by_id(meal.id());
        assert_eq!(result, meal.price().to_owned());
    }

    #[test]
    fn meal_not_found() {
        let extractor = AM::new_am(MockMealExtractor::new());
        let get_meal_price = GetMealPriceUsingExtractor::new(extractor.clone());

        let meal_id = rnd_meal_id();

        assert_panic!( {get_meal_price.invoke(&meal_id);}, String, starts with &format!("Meal #{:?} not found", meal_id));

        extractor.lock_un().verify_invoked_get_by_id(&meal_id);
    }
}
