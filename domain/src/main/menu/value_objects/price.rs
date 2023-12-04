use std::ops::{Add, Mul};
use std::str::FromStr;

use bigdecimal::*;
use common::types::main::base::value_object::ValueObject;
use common::types::main::common::count::Count;
use common::types::main::errors::error::BusinessError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq, Default)]
#[non_exhaustive]
pub struct Price {
    value: BigDecimal,
}

impl Price {
    pub const SCALE: i64 = 2;

    pub fn add(&self, additional: Self) -> Self {
        let summ: BigDecimal = BigDecimal::from_str(additional.to_string_value().as_str())
            .unwrap()
            .add(BigDecimal::from_str(self.to_string_value().as_str()).unwrap());
        Self { value: summ }
    }

    pub fn multiple(&self, multiplicator: Count) -> Self {
        let count = multiplicator.to_i32();
        let current_price_value = self.to_f64();
        let multiplied_price_value = current_price_value.mul(count as f64).to_string();
        Self {
            value: BigDecimal::from_str(&multiplied_price_value).unwrap(),
        }
    }

    pub fn to_f64(&self) -> f64 {
        self.to_owned().value.to_f64().unwrap()
    }

    pub fn to_bigdecimal(&self) -> BigDecimal {
        self.to_owned().value
    }

    pub fn to_string_value(&self) -> String {
        self.to_owned().value.to_string()
    }

    pub fn zero() -> Self {
        Self {
            value: BigDecimal::zero(),
        }
    }
}

impl TryFrom<BigDecimal> for Price {
    type Error = CreatePriceError;

    fn try_from(value: BigDecimal) -> Result<Self, Self::Error> {
        let price_scale = value.normalized().into_bigint_and_exponent().1;
        match &value {
            _ if price_scale > Self::SCALE => Err(CreatePriceError::InvalidScale),
            _ if value < BigDecimal::zero() => Err(CreatePriceError::NegativeValue),
            _ => Ok(Self {
                value: value.with_scale(Self::SCALE),
            }),
        }
    }
}

impl ValueObject for Price {}

#[derive(Debug, PartialEq, Clone)]
pub enum CreatePriceError {
    InvalidScale,
    NegativeValue,
}

impl BusinessError for CreatePriceError {}
