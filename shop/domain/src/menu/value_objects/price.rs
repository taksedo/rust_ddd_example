use std::{
    ops::{Add, Mul},
    str::FromStr,
};

use bigdecimal::*;
use common::types::{base::ValueObject, common::Count, errors::BusinessError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq, Default)]
#[non_exhaustive]
pub struct Price(BigDecimal);

impl Price {
    pub const SCALE: i64 = 2;

    pub fn add(&self, additional: Self) -> Self {
        let summ: BigDecimal = BigDecimal::from_str(&additional.to_string_value())
            .unwrap()
            .add(BigDecimal::from_str(&self.to_string_value()).unwrap());
        Self(summ)
    }

    pub fn multiple(&self, multiplicator: Count) -> Self {
        let count = multiplicator.to_i32();
        let current_price_value = self.to_f64();
        let multiplied_price_value = current_price_value.mul(count as f64).to_string();
        Self(BigDecimal::from_str(&multiplied_price_value).unwrap())
    }

    pub fn to_f64(&self) -> f64 {
        self.to_owned().0.to_f64().unwrap()
    }

    pub fn to_bigdecimal(&self) -> BigDecimal {
        self.to_owned().0
    }

    pub fn to_string_value(&self) -> String {
        self.to_owned().0.to_string()
    }

    pub fn zero() -> Self {
        Self(BigDecimal::zero())
    }
}

impl TryFrom<BigDecimal> for Price {
    type Error = CreatePriceError;

    fn try_from(value: BigDecimal) -> Result<Self, Self::Error> {
        let price_scale = value.normalized().into_bigint_and_exponent().1;
        match &value {
            _ if price_scale > Self::SCALE => Err(Self::Error::InvalidScale),
            _ if value < BigDecimal::zero() => Err(Self::Error::NegativeValue),
            _ => Ok(Self(value.with_scale(Self::SCALE))),
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

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn create_price__success(#[values(0_u64, 1_u64)] value: u64) {
        let input = BigDecimal::from(value);
        let price = Price::try_from(input.to_owned()).unwrap();
        assert_eq!(price.to_bigdecimal(), input.with_scale(2));
    }

    #[test]
    fn create_price__change_scale() {
        let value = BigDecimal::from_str("1.4").unwrap();
        let price = Price::try_from(value).unwrap();
        assert_eq!(price.to_bigdecimal(), BigDecimal::from_str("1.40").unwrap())
    }

    #[test]
    fn create_price__invalid_scale() {
        let price = BigDecimal::from_str("1.411").unwrap();
        let result = Price::try_from(price);
        assert_eq!(result, Err(CreatePriceError::InvalidScale));
    }

    #[test]
    fn create_price__negative_value() {
        let price = BigDecimal::from(-1);
        let result = Price::try_from(price);
        assert_eq!(result, Err(CreatePriceError::NegativeValue));
    }

    #[test]
    fn add_price() {
        let price1 = Price::try_from(BigDecimal::from_str("1.44").unwrap()).unwrap();
        dbg!(&price1);
        let price2 = Price::try_from(BigDecimal::from_str("1.45").unwrap()).unwrap();
        dbg!(&price2);

        let result = price1.add(price2);
        dbg!(&result);
        assert_eq!(
            result.to_bigdecimal(),
            BigDecimal::from_str("2.89").unwrap()
        );
    }

    #[test]
    fn multiple_to_count() {
        let price = Price::try_from(BigDecimal::from_str("1.5").unwrap()).unwrap();
        let count = Count::try_from(3).unwrap();
        let result = price.multiple(count);
        assert_eq!(
            result.to_bigdecimal(),
            BigDecimal::from_str("4.50").unwrap()
        );
    }

    #[test]
    fn format_as_string() {
        let priceStr = "111111122222222222";
        let price = Price::try_from(BigDecimal::from_str(priceStr).unwrap()).unwrap();

        let left_result = price.to_string_value();
        let right_result = format!("{}.00", priceStr);

        assert_eq!(left_result, right_result);
    }
}
