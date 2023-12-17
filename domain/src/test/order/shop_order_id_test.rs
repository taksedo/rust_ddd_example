use rand::{thread_rng, Rng};

use crate::main::order::{shop_order::ShopOrderError, value_objects::shop_order_id::ShopOrderId};

#[test]
fn check_equality() {
    let id: i64 = thread_rng().gen_range(0..i64::MAX);

    dbg!(&id);
    let shop_order_id_1 = ShopOrderId::try_from(id).unwrap();
    let shop_order_id_2 = ShopOrderId::try_from(id).unwrap();
    assert_eq!(shop_order_id_1, shop_order_id_2);
    assert_eq!(shop_order_id_1.to_i64(), shop_order_id_2.to_i64());
}

#[test]
fn wrong_id_value() {
    let id = thread_rng().gen_range(i64::MIN..0);

    let shop_order_id = ShopOrderId::try_from(id);

    assert_eq!(
        shop_order_id.unwrap_err(),
        ShopOrderError::IdGenerationError
    );
}
