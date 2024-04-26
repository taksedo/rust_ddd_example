use serde_derive::{Deserialize, Serialize};

use crate::{
    base::value_object::ValueObject,
    common::address::CreateAddressError::{EmptyString, NonPositiveBuilding},
};

/// `Address` Value Object
#[derive(Debug, Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
pub struct Address {
    street: String,
    building: i16,
}

impl ValueObject for Address {}

impl Address {
    /// Get street name from [Address]
    pub fn street_to_string(&self) -> String {
        self.street.clone()
    }
    /// Get building number from [Address]
    pub fn building_to_i16(&self) -> i16 {
        self.building
    }
}

impl TryFrom<(&str, i16)> for Address {
    type Error = CreateAddressError;

    fn try_from(value: (&str, i16)) -> Result<Self, Self::Error> {
        match value {
            (x, _) if x.is_empty() || x == " " => Err(EmptyString),
            (_, ..=0) => Err(NonPositiveBuilding),
            _ => Ok(Self {
                street: value.0.to_owned(),
                building: value.1,
            }),
        }
    }
}

/// Number of errors for [Address]
#[derive(Debug, Eq, PartialEq)]
pub enum CreateAddressError {
    /// Empty string error
    EmptyString,
    /// Building number less or equal than Zero
    NonPositiveBuilding,
}

#[cfg(test)]
mod test {
    use fake::{
        faker::address::en::{BuildingNumber, StreetName},
        Fake,
    };
    use rstest::rstest;

    use super::*;

    #[test]
    fn create_address_success() {
        let street = &*StreetName().fake::<String>();
        let str_building_number = BuildingNumber().fake::<String>();
        let building = if let Ok(unwrapped_value) = str_building_number.parse::<i16>() {
            unwrapped_value
        } else {
            i16::MAX
        };

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
}
