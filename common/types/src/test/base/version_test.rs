use crate::main::base::domain_entity::Version;
use rand::thread_rng;
use rand::Rng;
use std::convert::From;

#[test]
#[allow(non_snake_case)]
fn new_id__check_version_is_zero() {
    let first_version = Version::new();
    let second_version = Version::new();

    assert_eq!(first_version.to_u64(), second_version.to_u64());
    assert_eq!(first_version.to_u64(), 0)
}

#[test]
fn restore_from_long() {
    let long: u64 = thread_rng().gen();
    let version = Version::from(long);
    assert_eq!(version.to_u64(), long)
}

#[test]
#[allow(non_snake_case)]
fn increment_counter__value_is_plus_1() {
    let long: u64 = thread_rng().gen();
    let version = Version::from(long);
    let incremented = version.next();
    assert_eq!(incremented.to_u64(), long + 1)
}

#[test]
#[allow(non_snake_case)]
fn the_same_value_should_be_equals() {
    let long: u64 = thread_rng().gen();
    let first = Version::from(long);
    let second = Version::from(long);
    assert_eq!(first, second)
}

#[test]
#[allow(non_snake_case)]
fn previous_version_should_be_current_minus_1() {
    let long: u64 = thread_rng().gen();
    let version = Version::from(long);
    let previous = version.previous();
    assert_eq!(previous.to_u64(), version.to_u64() - 1)
}
