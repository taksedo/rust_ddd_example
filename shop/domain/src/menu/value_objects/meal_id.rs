use std::fmt::Debug;

use bigdecimal::ToPrimitive;
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Default, Eq, Hash, Display)]
#[non_exhaustive]
pub struct MealId(i64);

impl MealId {
    pub fn to_i64(&self) -> i64 {
        self.0.to_i64().unwrap()
    }
}

impl TryFrom<i64> for MealId {
    type Error = MealIdError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0..=i64::MAX => Ok(Self(value)),
            _ => Err(Self::Error::IdGenerationError),
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

#[cfg(test)]
mod tests {
    use rand::{Rng, rng};

    use super::*;

    #[test]
    fn check_equality() {
        let id: i64 = rng().random_range(0..i64::MAX);

        dbg!(&id);
        let meal_id1 = MealId::try_from(id).unwrap();
        let meal_id2 = MealId::try_from(id).unwrap();
        assert_eq!(meal_id1, meal_id2);
        assert!(!std::ptr::eq(&meal_id1, &meal_id2)); //check not same instance
        assert_eq!(meal_id1.to_i64(), meal_id2.to_i64());
    }

    #[test]
    fn wrong_id_value() {
        let id = rng().random_range(i64::MIN..0);

        let meal_id = MealId::try_from(id);

        assert_eq!(meal_id.unwrap_err(), MealIdError::IdGenerationError);
    }
}
