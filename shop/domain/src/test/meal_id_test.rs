use rand::{thread_rng, Rng};

use crate::main::menu::value_objects::meal_id::MealId;

#[test]
fn check_equality() {
    let id: i64 = thread_rng().gen_range(0..i64::MAX);

    dbg!(&id);
    let meal_id1 = MealId::try_from(id).unwrap();
    let meal_id2 = MealId::try_from(id).unwrap();
    assert_eq!(meal_id1, meal_id2);
    // todo забороть same instance assert_ne!(meal_id1.type_id(), meal_id2.type_id());
    assert_eq!(meal_id1.to_i64(), meal_id2.to_i64());
}
