use std::fmt::Debug;

use bigdecimal::ToPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Default, Eq, Hash)]
#[non_exhaustive]
pub struct MealId {
    value: i64,
}

impl MealId {
    pub fn to_i64(&self) -> i64 {
        self.value.to_i64().unwrap()
    }
}

impl TryFrom<i64> for MealId {
    type Error = MealIdError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            x if x > 0 && x < i64::MAX => Ok(MealId { value }),
            _ => Err(MealIdError::IdGenerationError),
        }
    }
}

pub trait MealIdGenerator: Debug + Send {
    fn generate(&mut self) -> MealId;
}

#[derive(Debug, PartialEq)]
pub enum MealIdError {
    IdGenerationError,
}
