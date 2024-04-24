use std::{fmt, fmt::Formatter};

use common::types::base::value_object::ValueObject;
use derive_new::new;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(new, Debug, Clone, Deserialize, Serialize, PartialEq, Default, Eq, Hash, Copy)]
pub struct CustomerId(#[new(value = "Uuid::new_v4()")] Uuid);

impl From<Uuid> for CustomerId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl fmt::Display for CustomerId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl ValueObject for CustomerId {}
