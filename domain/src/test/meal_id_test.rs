use crate::main::menu::meal_id::MealId;
use rand::{thread_rng, Rng};

#[test]
fn check_equality() {
    let id: u64 = thread_rng().gen();

    let meal_id1 = MealId::new(id);
    let meal_id2 = MealId::new(id);
    assert_eq!(meal_id1, meal_id2);
    // todo забороть same instance assert_ne!(meal_id1.type_id(), meal_id2.type_id());
    assert_eq!(meal_id1.to_u64(), meal_id2.to_u64());
}
