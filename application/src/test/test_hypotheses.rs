use std::sync::atomic::{AtomicI64, Ordering};

use derive_new::new;

use domain::main::menu::value_objects::meal_id::{MealId, MealIdGenerator};

#[derive(Debug, new)]
pub struct InMemoryIncrementalMealIdGenerator {
    #[new(value = "AtomicI64::from(1)")]
    counter: AtomicI64,
}

impl MealIdGenerator for InMemoryIncrementalMealIdGenerator {
    fn generate(&mut self) -> MealId {
        let meal_id = self.counter.fetch_add(1, Ordering::SeqCst);
        MealId::try_from(meal_id).unwrap()
    }
}

#[test]
fn test() {
    let mut counter = InMemoryIncrementalMealIdGenerator::new();
    let _c = counter.generate();

    let _c = counter.generate();
}
