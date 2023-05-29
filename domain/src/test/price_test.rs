#![allow(non_snake_case)]

use crate::main::menu::price::{CreatePriceError, Price};
use bigdecimal::BigDecimal;
use common_types::main::common::count::Count;
use rstest::rstest;
use std::str::FromStr;

#[rstest]
#[case(0_i64)]
#[case(1_i64)]
fn create_price__success(#[case] value: i64) {
    let input = BigDecimal::from(value);
    let price = Price::from(input.to_owned()).unwrap();
    assert_eq!(price.to_bigdecimal_value(), input.with_scale(2));
}

#[test]
fn create_price__change_scale() {
    let value = BigDecimal::from_str("1.4").unwrap();
    let price = Price::from(value).unwrap();

    assert_eq!(
        price.to_bigdecimal_value(),
        BigDecimal::from_str("1.40").unwrap()
    )
}

#[test]
fn create_price__invalid_scale() {
    let price = BigDecimal::from_str("1.411").unwrap();
    let result = Price::from(price);

    assert_eq!(result, Err(CreatePriceError::InvalidScale));
}

#[test]
fn create_price__negative_value() {
    let price = BigDecimal::from(-1);
    let result = Price::from(price);

    assert_eq!(result, Err(CreatePriceError::NegativeValue));
}

#[test]
fn add_price() {
    let price1 = Price::from(BigDecimal::from_str("1.44").unwrap()).unwrap();
    let price2 = Price::from(BigDecimal::from_str("1.45").unwrap()).unwrap();

    let result = price1.add(price2);
    assert_eq!(
        result.to_bigdecimal_value(),
        BigDecimal::from_str("2.89").unwrap()
    );
}

#[test]
fn multiple_to_count() {
    let price = Price::from(BigDecimal::from_str("1.5").unwrap()).unwrap();
    let count = Count::from(3).unwrap();
    let result = price.multiple(count);
    assert_eq!(
        result.to_bigdecimal_value(),
        BigDecimal::from_str("4.50").unwrap()
    );
}

#[test]
fn format_as_string() {
    let priceStr = "111111122222222222";
    let price = Price::from(BigDecimal::from_str(priceStr).unwrap()).unwrap();

    let left_result = price.to_string_value();
    let right_result = format!("{}.00", priceStr);

    assert_eq!(left_result, right_result);
}
