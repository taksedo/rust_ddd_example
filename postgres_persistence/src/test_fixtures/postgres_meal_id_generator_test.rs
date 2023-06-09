use crate::main::postgres_meal_id_generator::PostgresMealIdGenerator;
use domain::main::menu::meal_id::{MealId, MealIdGenerator};
use domain::test_fixtures::fixtures::rnd_meal_id;

#[test]
fn generate_id() {
    let id = rnd_meal_id();
    let id_generator = PostgresMealIdGenerator::new(id);
    let meal_id = id_generator.generate();

    assert_eq!(meal_id, MealId::new(id.to_i64() + 1));
}
