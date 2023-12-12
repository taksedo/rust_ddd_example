use domain::main::order::value_objects::shop_order_id::ShopOrderIdGenerator;

use crate::main::order::in_memory_incremental_shop_order_id_generator::InMemoryIncrementalShopOrderIdGenerator;

#[test]
fn id_is_incremented() {
    let mut generator = InMemoryIncrementalShopOrderIdGenerator::new();
    let order_id_1 = generator.generate();
    let order_id_2 = generator.generate();
    assert_eq!(order_id_1.to_i64(), order_id_2.to_i64() - 1);
}
