use std::fmt::Debug;

use serde_derive::{Deserialize, Serialize};

use crate::main::cart::cart::CartError;

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Default, Eq, Hash)]
#[non_exhaustive]
pub struct CartId(i64);

impl CartId {
    pub fn to_i64(&self) -> i64 {
        self.0
    }
}

pub trait CartIdGenerator: Debug + Send {
    fn generate(&mut self) -> CartId;
}

impl TryFrom<i64> for CartId {
    type Error = CartError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            x if x > 0 && x < i64::MAX => Ok(Self(value)),
            _ => Err(Self::Error::IdGenerationError),
        }
    }
}
