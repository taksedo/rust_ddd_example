use bigdecimal::*;
use common_types::main::base::value_object::ValueObject;
use common_types::main::common::count::Count;
use common_types::main::errors::error::BusinessError;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul};
use std::str::FromStr;

pub const SCALE: i64 = 2;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Price {
    pub value: BigDecimal,
}

impl Price {
    pub fn add(&self, additional: Self) -> Self {
        let summ: BigDecimal = BigDecimal::from_str(additional.to_string_value().as_str())
            .unwrap()
            .add(BigDecimal::from_str(&self.to_string_value().as_str()).unwrap());
        Self { value: summ }
    }

    pub fn multiple(&self, multiplicator: Count) -> Self {
        let count = multiplicator.value.to_i64().unwrap();
        let current_price_value = self.to_f64();
        let multiplied_price_value = current_price_value.mul(count as f64);
        Self {
            value: BigDecimal::from_f64(multiplied_price_value).unwrap(),
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

    fn _zero(&self) -> Self {
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
            _ if price_scale > SCALE => Err(CreatePriceError::InvalidScale),
            _ if value < BigDecimal::zero() => Err(CreatePriceError::NegativeValue),
            _ => Ok(Self {
                value: value.with_scale(SCALE),
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
