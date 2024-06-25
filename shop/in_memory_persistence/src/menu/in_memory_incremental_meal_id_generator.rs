use std::sync::Mutex;

use derive_new::new;
use domain::menu::value_objects::meal_id::{MealId, MealIdGenerator};

#[derive(Debug, new)]
pub struct InMemoryIncrementalMealIdGenerator {
    #[new(value = "Mutex::new(0)")]
    counter: Mutex<i64>,
}

impl MealIdGenerator for InMemoryIncrementalMealIdGenerator {
    fn generate(&mut self) -> MealId {
        let mut meal_id = self.counter.lock().unwrap();
        *meal_id += 1;
        MealId::try_from(*meal_id).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_is_incremented() {
        let mut generator = InMemoryIncrementalMealIdGenerator::new();
        let meal_id1 = generator.generate();
        let meal_id2 = generator.generate();
        assert_eq!(meal_id1.to_i64(), meal_id2.to_i64() - 1);
    }
}
