use derive_new::new;

#[derive(new, Eq, PartialEq, Debug)]
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
