use std::convert::From;

use rand::random;

use crate::main::base::domain_entity::Version;

#[test]
#[allow(non_snake_case)]
fn new_id__check_version_is_zero() {
    let first_version = Version::new();
    let second_version = Version::new();

    assert_eq!(first_version.to_i64(), second_version.to_i64());
    assert_eq!(first_version.to_i64(), 0)
}

#[test]
fn restore_from_long() {
    let long: i64 = random();
    let version = Version::from(long);
    assert_eq!(version.to_i64(), long)
}

#[test]
#[allow(non_snake_case)]
fn increment_counter__value_is_plus_1() {
    let long: i64 = random();
    let version = Version::from(long);
    let incremented = version.next();
    assert_eq!(incremented.to_i64(), long + 1)
}

#[test]
#[allow(non_snake_case)]
fn the_same_value_should_be_equals() {
    let long: i64 = random();
    let first = Version::from(long);
    let second = Version::from(long);
    assert_eq!(first, second)
}

#[test]
#[allow(non_snake_case)]
fn previous_version_should_be_current_minus_1() {
    let long: i64 = random();
    let version = Version::from(long);
    let previous = version.previous();
    assert_eq!(previous.to_i64(), version.to_i64() - 1)
}
