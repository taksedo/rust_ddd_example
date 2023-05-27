use actix_web::ResponseError;
use bigdecimal::*;
use common_types::main::base::value_object::ValueObject;
use common_types::main::common::count::Count;
use common_types::main::errors::error::BusinessError;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Price {
    pub value: BigDecimal,
}

impl Price {
    pub fn from(value: BigDecimal) -> Result<Price, CreatePriceError> {
        let price_scale = value.clone().into_bigint_and_exponent().1;
        match &value {
            _ if price_scale > Self::SCALE as i64 => Err(CreatePriceError::InvalidScale),
            _ if value < BigDecimal::zero() => Err(CreatePriceError::NegativeValue),
            _ => Ok(Self {
                value: value.with_scale(Self::SCALE as i64),
            }),
        }
    }

    pub fn add(&self, additional: Self) -> Self {
        let additional_price_value = additional.to_f64_value();
        let current_price_value = &self.to_f64_value();
        let total_price_value = current_price_value.add(additional_price_value);
        Self {
            value: BigDecimal::from_f64(total_price_value).unwrap(),
        }
    }

    pub fn multiple(&self, multiplicator: Count) -> Self {
        let count = multiplicator.value.to_f64().unwrap();
        let current_price_value = &self.to_f64_value();
        let multiplied_price_value = current_price_value.mul(count);
        Self {
            value: BigDecimal::from_f64(multiplied_price_value).unwrap(),
        }
    }

    pub fn to_f64_value(&self) -> f64 {
        self.to_owned().value.to_f64().unwrap()
    }

    pub fn to_bigdecimal_value(&self) -> BigDecimal {
        self.to_owned().value
    }

    pub fn to_string_value(&self) -> String {
        self.to_owned().value.to_string()
    }

    const SCALE: i8 = 2;

    fn _zero(&self) -> Self {
        Self {
            value: BigDecimal::zero(),
        }
    }
}

impl ValueObject for Price {}

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum CreatePriceError {
    #[error("Неправильное количество знаков после запятой")]
    InvalidScale,
    #[error("Отрицательное значение")]
    NegativeValue,
}

impl BusinessError for CreatePriceError {}

impl ResponseError for CreatePriceError {}
