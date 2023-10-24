#![allow(non_snake_case)]

use crate::main::common::count::{Count, CountError};
use rstest::rstest;

#[rstest]
fn create_count__success(#[values(0_i32, 1_i32, i32::MAX)] value: i32) {
    let result = Count::from(value);
    let count = result.unwrap();
    assert_eq!(count.to_i32(), value);
}

#[test]
fn create_count__one() {
    let result = Count::one();
    assert_eq!(result.to_i32(), 1)
}

#[test]
#[allow(non_snake_case)]
fn create_count__negative_value() {
    let result = Count::from(-(1));
    assert_eq!(result, Err(CountError::NegativeValueError));
}

#[test]
fn increment__success() {
    let count = Count::from(1).unwrap();
    let increment = count.increment();
    assert_eq!(increment, Count::from(count.to_i32() + 1));
}

#[test]
fn increment__max_value_reached() {
    let count = Count::from(i32::MAX).unwrap();
    let result = count.increment();
    assert_eq!(result, Err(CountError::MaxValueReachedError));
}

#[rstest]
fn decrement__success(#[values(1_i32, i32::MAX)] value: i32) {
    let count = Count::from(value).unwrap();
    let increment = count.decrement();
    assert_eq!(increment, Count::from(count.to_i32() - 1));
}

#[test]
fn decrement__min_value_reached() {
    let count = Count::from(0).unwrap();
    let result = count.decrement();
    assert_eq!(result, Err(CountError::MinValueReachedError));
}

#[test]
fn check_is_min_value__true() {
    let count = Count::from(0).unwrap();
    assert!(count.is_min());
}

#[test]
fn check_is_min_value__false() {
    let count = Count::from(1).unwrap();
    assert!(!count.is_min());
}

#[test]
fn check_is_max_value__true() {
    let count = Count::from(i32::MAX).unwrap();
    assert!(count.is_max());
}

#[test]
fn check_is_max_value__false() {
    let count = Count::from(1).unwrap();
    assert!(!count.is_max());
}
