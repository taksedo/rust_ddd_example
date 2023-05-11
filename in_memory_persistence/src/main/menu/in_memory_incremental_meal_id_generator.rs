use derive_new::new;
use domain::main::menu::meal_id::{MealId, MealIdGenerator};
use std::sync::atomic::{AtomicI64, Ordering};

#[derive(Debug, new)]
pub struct InMemoryIncrementalMealIdGenerator {
    #[new(value = "AtomicI64::from(0)")]
    counter: AtomicI64,
}

impl MealIdGenerator for InMemoryIncrementalMealIdGenerator {
    fn generate(&self) -> MealId {
        let meal_id = self.counter.fetch_add(1, Ordering::SeqCst);
        MealId { value: meal_id }
    }
}
