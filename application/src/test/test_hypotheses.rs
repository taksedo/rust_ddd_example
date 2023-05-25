use derive_new::new;
use domain::main::menu::meal_id::{MealId, MealIdGenerator};
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, new)]
pub struct InMemoryIncrementalMealIdGenerator {
    #[new(value = "AtomicU64::from(1)")]
    counter: AtomicU64,
}

impl MealIdGenerator for InMemoryIncrementalMealIdGenerator {
    fn generate(&mut self) -> MealId {
        let meal_id = self.counter.fetch_add(1, Ordering::SeqCst);
        MealId { value: meal_id }
    }
}

#[test]
fn test() {
    let mut counter = InMemoryIncrementalMealIdGenerator::new();
    let c = counter.generate();
    dbg!(c);
    let c = counter.generate();
    dbg!(c);
}
