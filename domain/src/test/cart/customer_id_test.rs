use uuid::Uuid;

use crate::main::cart::value_objects::customer_id::CustomerId;

#[test]
fn check_equality() {
    let id = Uuid::new_v4();

    let customer_id1 = CustomerId::from(id);
    let customer_id2 = CustomerId::from(id);

    assert_eq!(customer_id1, customer_id2);
    assert_eq!(customer_id1.to_string(), customer_id2.to_string());
}
