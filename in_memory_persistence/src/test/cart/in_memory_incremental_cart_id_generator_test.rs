use crate::main::cart::in_memory_incremental_cart_id_generator::InMemoryIncrementalCartIdGenerator;
use domain::main::cart::value_objects::cart_id::CartIdGenerator;

#[test]
fn id_is_incremented() {
    let mut id_generator = InMemoryIncrementalCartIdGenerator::new();
    let cart_id1 = id_generator.generate();
    let cart_id2 = id_generator.generate();
    assert_eq!(cart_id1.to_i64(), cart_id2.to_i64() - 1);
}
