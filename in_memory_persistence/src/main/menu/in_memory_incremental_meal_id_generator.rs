use derive_new::new;
use domain::main::menu::meal_id::{MealId, MealIdGenerator};
use std::sync::Mutex;

#[derive(Debug, new)]
pub struct InMemoryIncrementalMealIdGenerator {
    #[new(value = "Mutex::new(0)")]
    counter: Mutex<i64>,
}

impl MealIdGenerator for InMemoryIncrementalMealIdGenerator {
    fn generate(&mut self) -> MealId {
        let mut meal_id = self.counter.lock().unwrap();
        *meal_id += 1;
        MealId { value: *meal_id }
    }
}
