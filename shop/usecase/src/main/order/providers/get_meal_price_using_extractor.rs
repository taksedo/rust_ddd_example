use std::sync::{Arc, Mutex};

use derive_new::new;
use domain::{
    menu::value_objects::{meal_id::MealId, price::Price},
    order::get_meal_price::GetMealPrice,
};

use crate::main::menu::access::meal_extractor::MealExtractor;

impl GetMealPrice for GetMealPriceUsingExtractor {
    fn invoke(&self, for_meal_id: &MealId) -> Price {
        let meal = &self.extractor.lock().unwrap().get_by_id(for_meal_id);
        assert!(meal.is_some(), "Meal #{:?} not found", for_meal_id);
        meal.clone().unwrap().price().clone()
    }
}

#[derive(new, Debug)]
pub struct GetMealPriceUsingExtractor {
    pub extractor: Arc<Mutex<dyn MealExtractor>>,
}

#[cfg(test)]
mod tests {
    use assert_panic::assert_panic;
    use domain::test_fixtures::{rnd_meal, rnd_meal_id};

    use super::*;
    use crate::test_fixtures::MockMealExtractor;

    #[test]
    fn price_has_been_provided() {
        let meal = rnd_meal();

        let extractor = Arc::new(Mutex::new(MockMealExtractor::new()));
        extractor.lock().unwrap().meal = Some(meal.clone());

        let get_meal_price = GetMealPriceUsingExtractor::new(extractor.clone());
        let result = get_meal_price.invoke(meal.id());

        extractor
            .lock()
            .unwrap()
            .verify_invoked_get_by_id(&meal.id());
        assert_eq!(result, meal.price().to_owned());
    }

    #[test]
    fn meal_not_found() {
        let extractor = Arc::new(Mutex::new(MockMealExtractor::new()));
        let get_meal_price = GetMealPriceUsingExtractor::new(extractor.clone());

        let meal_id = rnd_meal_id();

        assert_panic!( {get_meal_price.invoke(&meal_id);}, String, starts with &format!("Meal #{:?} not found", meal_id));

        extractor.lock().unwrap().verify_invoked_get_by_id(&meal_id);
    }
}
