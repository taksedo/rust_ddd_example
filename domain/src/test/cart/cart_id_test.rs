use rand::{thread_rng, Rng};

use crate::main::cart::cart::CartError;
use crate::main::cart::value_objects::cart_id::CartId;

#[test]
fn check_equality() {
    let id = thread_rng().gen_range(0..i64::MAX);

    let cart_id1 = CartId::try_from(id).unwrap();
    let cart_id2 = CartId::try_from(id).unwrap();

    assert_eq!(cart_id1, cart_id1);
    assert_eq!(cart_id1.to_i64(), cart_id2.to_i64())
}

#[test]
fn wrong_id_value() {
    let id = thread_rng().gen_range(i64::MIN..0);

    let cart_id = CartId::try_from(id);

    assert_eq!(cart_id.unwrap_err(), CartError::IdGenerationError);
}
