use std::sync::{Arc, Mutex};

use domain::main::menu::value_objects::meal_id::MealId;
use domain::main::menu::value_objects::price::Price;
use domain::main::order::get_meal_price::GetMealPrice;

use crate::main::menu::access::meal_extractor::MealExtractor;

pub struct GetMealPriceUsingExtractor {
    pub extractor: Arc<Mutex<dyn MealExtractor>>,
}

impl GetMealPrice for GetMealPriceUsingExtractor {
    fn invoke(&self, for_meal_id: MealId) -> Price {
        let meal = &self.extractor.lock().unwrap().get_by_id(for_meal_id);
        if meal.is_some() {
            println!("Meal #{:?} not found", for_meal_id);
        }
        meal.clone().unwrap().price
    }
}
