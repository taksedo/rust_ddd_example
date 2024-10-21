use std::sync::Mutex;

use derive_new::new;
use domain::order::value_objects::shop_order_id::{ShopOrderId, ShopOrderIdGenerator};

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

#[cfg(all(test, feature = "in_memory_persistence"))]
mod tests {
    use super::*;

    #[test]
    fn id_is_incremented() {
        let mut generator = InMemoryIncrementalShopOrderIdGenerator::new();
        let order_id_1 = generator.generate();
        let order_id_2 = generator.generate();
        assert_eq!(order_id_1.to_i64(), order_id_2.to_i64() - 1);
    }
}
