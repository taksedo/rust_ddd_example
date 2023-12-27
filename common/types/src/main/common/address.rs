use serde_derive::{Deserialize, Serialize};

use crate::main::{
    base::value_object::ValueObject,
    common::address::CreateAddressError::{EmptyString, NonPositiveBuilding},
};

#[derive(Debug, Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
pub struct Address {
    street: String,
    building: i16,
}

impl ValueObject for Address {}

impl Address {
    pub fn street_to_string(&self) -> String {
        self.street.clone()
    }

    pub fn building_to_i16(&self) -> i16 {
        self.building
    }
}

impl TryFrom<(&str, i16)> for Address {
    type Error = CreateAddressError;

    fn try_from(value: (&str, i16)) -> Result<Self, Self::Error> {
        match value {
            (x, _) if x.is_empty() || x == " " => Err(EmptyString),
            (_, x) if x <= 0 => Err(NonPositiveBuilding),
            _ => Ok(Self {
                street: value.0.to_owned(),
                building: value.1,
            }),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum CreateAddressError {
    EmptyString,
    NonPositiveBuilding,
}
