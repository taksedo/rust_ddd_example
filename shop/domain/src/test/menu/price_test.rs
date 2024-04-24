#![allow(non_snake_case)]

use std::str::FromStr;

use bigdecimal::BigDecimal;
use common::types::common::count::Count;
use rstest::rstest;

use crate::main::menu::value_objects::price::{CreatePriceError, Price};

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
