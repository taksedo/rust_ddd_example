use crate::main::menu::in_memory_incremental_meal_id_generator::InMemoryIncrementalMealIdGenerator;
use domain::main::menu::meal_id::MealIdGenerator;

#[test]
fn id_is_incremented() {
    let generator = InMemoryIncrementalMealIdGenerator::new();
    let meal_id1 = generator.generate();
    let meal_id2 = generator.generate();
    assert_eq!(meal_id1.value, meal_id2.value - 1);
}
