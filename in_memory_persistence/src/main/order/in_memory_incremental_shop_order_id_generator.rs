use std::sync::Mutex;

use derive_new::new;

use domain::main::order::value_objects::shop_order_id::{ShopOrderId, ShopOrderIdGenerator};

#[derive(Debug, new)]
pub struct InMemoryIncrementalShopOrderIdGenerator {
    #[new(value = "Mutex::new(0)")]
    counter: Mutex<i64>,
}

impl ShopOrderIdGenerator for InMemoryIncrementalShopOrderIdGenerator {
    fn generate(&mut self) -> ShopOrderId {
        let mut order_id = self.counter.lock().unwrap();
        *order_id += 1;
        ShopOrderId::try_from(*order_id).unwrap()
    }
}
