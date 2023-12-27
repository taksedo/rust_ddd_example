use fake::{
    faker::address::en::{BuildingNumber, StreetName},
    Fake,
};
use rstest::rstest;

use crate::main::common::address::{
    Address,
    CreateAddressError::{EmptyString, NonPositiveBuilding},
};

#[test]
fn create_address_success() {
    let street = &*StreetName().fake::<String>();
    let str_building_number = BuildingNumber().fake::<String>();
    let building = str_building_number.parse::<i16>().unwrap();

    let result = Address::try_from((street, building));

    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.building_to_i16(), building);
    assert_eq!(result.street_to_string(), street);
}

#[rstest]
fn create_address_empty_string(#[values("", " ")] value: &str) {
    let result = Address::try_from((value, 15_i16));

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), EmptyString);
}

#[rstest]
fn create_address_non_positive_building(#[values(0, - 1)] value: i16) {
    let result = Address::try_from(("Street", value));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), NonPositiveBuilding)
}
