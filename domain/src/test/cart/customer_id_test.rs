use uuid::Uuid;

use crate::main::cart::value_objects::customer_id::CustomerId;

#[test]
fn check_equality() {
    let id = Uuid::new_v4().to_string();

    let customer_id1 = CustomerId::new(id.clone());
    let customer_id2 = CustomerId::new(id);

    assert_eq!(customer_id1, customer_id2);
    assert_eq!(customer_id1.to_string(), customer_id2.to_string());
}
