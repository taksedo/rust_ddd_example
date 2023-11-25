use domain::main::menu::value_objects::meal_id::MealIdGenerator;

use crate::main::menu::in_memory_incremental_meal_id_generator::InMemoryIncrementalMealIdGenerator;

#[test]
fn id_is_incremented() {
    let mut generator = InMemoryIncrementalMealIdGenerator::new();
    let meal_id1 = generator.generate();
    let meal_id2 = generator.generate();
    assert_eq!(meal_id1.to_i64(), meal_id2.to_i64() - 1);
}
