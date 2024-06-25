use std::sync::{Arc, Mutex};

use derive_new::new;
use domain::main::menu::{
    meal_already_exists::MealAlreadyExists, value_objects::meal_name::MealName,
};

use crate::main::menu::access::meal_extractor::MealExtractor;

#[derive(new, Debug, Clone)]
pub struct MealAlreadyExistsUsesMealExtractor {
    pub extractor: Arc<Mutex<dyn MealExtractor>>,
}

impl MealAlreadyExists for MealAlreadyExistsUsesMealExtractor {
    fn invoke(&mut self, name: &MealName) -> bool {
        let meal_found_by_get = self.extractor.lock().unwrap().get_by_name(name);
        let meal_found_by_get_is_removed = *meal_found_by_get.clone().unwrap_or_default().removed();
        meal_found_by_get.is_some() & !meal_found_by_get_is_removed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_fixtures::{removed_meal, MockMealExtractor};
    use domain::test_fixtures::{rnd_meal, rnd_meal_name};

    #[test]
    fn meal_already_exists() {
        let meal = rnd_meal();
        let extractor = Arc::new(Mutex::new(MockMealExtractor {
            meal: Some(meal.to_owned()),
            ..MockMealExtractor::default()
        }));
        let mut rule = MealAlreadyExistsUsesMealExtractor::new(extractor);

        let result = rule.invoke(&meal.name());

        assert!(result);

        rule.extractor
            .lock()
            .unwrap()
            .downcast_ref::<MockMealExtractor>()
            .unwrap()
            .verify_invoked_get_by_name(&meal.name());
    }

    #[test]
    fn meal_already_exists_but_removed() {
        let meal = removed_meal();
        let extractor = Arc::new(Mutex::new(MockMealExtractor {
            meal: Some(meal.to_owned()),
            ..MockMealExtractor::default()
        }));
        let mut rule = MealAlreadyExistsUsesMealExtractor::new(extractor);

        let result = rule.invoke(&meal.name());

        assert!(!result);
        rule.extractor
            .lock()
            .unwrap()
            .downcast_ref::<MockMealExtractor>()
            .unwrap()
            .verify_invoked_get_by_name(&meal.name());
    }

    #[test]
    fn meal_already_exists_doesnt_exist() {
        let extractor = Arc::new(Mutex::new(MockMealExtractor::new()));
        let mut rule = MealAlreadyExistsUsesMealExtractor::new(extractor);

        let meal_name = rnd_meal_name();
        let result = rule.invoke(&meal_name);

        assert!(!result);
        rule.extractor
            .lock()
            .unwrap()
            .downcast_ref::<MockMealExtractor>()
            .unwrap()
            .verify_invoked_get_by_name(&meal_name);
    }
}
