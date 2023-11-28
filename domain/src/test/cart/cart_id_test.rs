use rand::{thread_rng, Rng};

use crate::main::order::shop_order_id::ShopOrderId;

#[test]
fn check_equality() {
    let id = thread_rng().gen_range(0..i64::MAX);

    let shop_order_id1 = ShopOrderId::new(id);
    let shop_order_id2 = ShopOrderId::new(id);

    assert_eq!(shop_order_id1, shop_order_id1);
    assert_eq!(
        shop_order_id1.to_long_value(),
        shop_order_id2.to_long_value()
    )
}
