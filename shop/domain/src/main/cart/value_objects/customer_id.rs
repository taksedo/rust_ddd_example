use common::types::base::value_object::ValueObject;
use derive_more::Display;
use derive_new::new;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    new, Debug, Clone, Deserialize, Serialize, PartialEq, Default, Eq, Hash, Copy, Display,
)]
#[non_exhaustive]
pub struct CustomerId(#[new(value = "Uuid::new_v4()")] Uuid);

impl From<Uuid> for CustomerId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl TryFrom<&str> for CustomerId {
    type Error = CustomerIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(uuid) = Uuid::parse_str(value) {
            Ok(Self(uuid))
        } else {
            Err(Self::Error::IdGenerationError)
        }
    }
}

impl ValueObject for CustomerId {}

#[derive(Debug, PartialEq)]
pub enum CustomerIdError {
    IdGenerationError,
}
