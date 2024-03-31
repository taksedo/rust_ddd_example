use std::fmt::Debug;

use bigdecimal::ToPrimitive;
use serde_derive::{Deserialize, Serialize};

use crate::main::order::shop_order::ShopOrderError;

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
