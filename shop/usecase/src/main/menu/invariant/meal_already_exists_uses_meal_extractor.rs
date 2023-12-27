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
        let meal_found_by_get = self.extractor.lock().unwrap().get_by_name(name.to_owned());
        let meal_found_by_get_is_removed = meal_found_by_get.clone().unwrap_or_default().removed;
        meal_found_by_get.is_some() & !meal_found_by_get_is_removed
    }
}
