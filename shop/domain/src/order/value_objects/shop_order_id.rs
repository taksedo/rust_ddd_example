use std::fmt::Debug;

use bigdecimal::ToPrimitive;
use serde_derive::{Deserialize, Serialize};

use crate::order::shop_order::ShopOrderError;

#[derive(
    Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Hash, Eq, Default, Ord, PartialOrd,
)]
#[non_exhaustive]
pub struct ShopOrderId(i64);

impl ShopOrderId {
    pub fn to_i64(&self) -> i64 {
        self.0.to_i64().unwrap()
    }
}

impl TryFrom<i64> for ShopOrderId {
    type Error = ShopOrderError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0..=i64::MAX => Ok(Self(value)),
            _ => Err(Self::Error::IdGenerationError),
        }
    }
}

pub trait ShopOrderIdGenerator: Debug + Send {
    fn generate(&mut self) -> ShopOrderId;
}

#[cfg(test)]
mod tests {
    use domain::order::{shop_order::ShopOrderError, value_objects::shop_order_id::ShopOrderId};
    use rand::random_range;

    #[test]
    fn check_equality() {
        let id: i64 = random_range(0..i64::MAX);

        dbg!(&id);
        let shop_order_id_1 = ShopOrderId::try_from(id).unwrap();
        let shop_order_id_2 = ShopOrderId::try_from(id).unwrap();
        assert_eq!(shop_order_id_1, shop_order_id_2);
        assert_eq!(shop_order_id_1.to_i64(), shop_order_id_2.to_i64());
    }

    #[test]
    fn wrong_id_value() {
        let id = random_range(i64::MIN..0);

        let shop_order_id = ShopOrderId::try_from(id);

        assert_eq!(
            shop_order_id.unwrap_err(),
            ShopOrderError::IdGenerationError
        );
    }
}
