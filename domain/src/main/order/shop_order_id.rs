use derive_new::new;
use serde_derive::{Deserialize, Serialize};

#[derive(new, Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq, Default)]
pub struct ShopOrderId {
    value: i64,
}

impl ShopOrderId {
    pub fn to_long_value(&self) -> i64 {
        self.value
    }
}

pub trait ShopOrderIdGenerator {
    fn generate() -> ShopOrderId;
}
