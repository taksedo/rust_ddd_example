#[allow(non_snake_case)]
use crate::main::common::count::{Count, CountError};
use rstest::rstest;

#[rstest]
#[case(0_u32)]
#[case(1_u32)]
#[case(u32::MAX)]
fn create_count_success(#[case] value: u32) {
    let result = Count::from(value);
    let count = result.unwrap();
    assert_eq!(count.to_u32_value(), value as u32);
}

#[test]
#[allow(non_snake_case)]
fn create_count__one() {
    let result = Count::one();
    assert_eq!(result.to_u32_value(), 1)
}

// #[test]
// #[allow(non_snake_case)]
// fn create_count__negative_value() {
//     let result = Count::from(-(1));
//     assert_eq!(result, Err(CountError::NegativeValueError));
// }

#[test]
#[allow(non_snake_case)]
fn increment__success() {
    let count = Count::from(1).unwrap();
    let increment = count.increment();
    assert_eq!(increment, Count::from(count.to_u32_value() + 1));
}

#[test]
#[allow(non_snake_case)]
fn increment__max_value_reached() {
    let count = Count::from(u32::MAX).unwrap();
    let result = count.increment();
    assert_eq!(result, Err(CountError::MaxValueReachedError));
}

#[rstest]
#[case(1_u32)]
#[case(u32::MAX)]
fn decrement_success(#[case] value: u32) {
    let count = Count::from(value).unwrap();
    let increment = count.decrement();
    assert_eq!(increment, Count::from(count.to_u32_value() - 1));
}

#[test]
#[allow(non_snake_case)]
fn decrement__min_value_reached() {
    let count = Count::from(0).unwrap();
    let result = count.decrement();
    assert_eq!(result, Err(CountError::MinValueReachedError));
}

#[test]
#[allow(non_snake_case)]
fn check_is_min_value__true() {
    let count = Count::from(0).unwrap();
    assert!(count.is_min());
}

#[test]
#[allow(non_snake_case)]
fn check_is_min_value__false() {
    let count = Count::from(1).unwrap();
    assert!(!count.is_min());
}

#[test]
#[allow(non_snake_case)]
fn check_is_max_value__true() {
    let count = Count::from(u32::MAX).unwrap();
    assert!(count.is_max());
}

#[test]
#[allow(non_snake_case)]
fn check_is_max_value__false() {
    let count = Count::from(1).unwrap();
    assert!(!count.is_max());
}
