use std::fmt::Debug;

use serde_derive::{Deserialize, Serialize};

use crate::cart::cart::CartError;

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
            0..=i64::MAX => Ok(Self(value)),
            _ => Err(Self::Error::IdGenerationError),
        }
    }
}

#[cfg(test)]
mod tests {
    use domain::cart::{cart::CartError, value_objects::cart_id::CartId};
    use rand::{thread_rng, Rng};

    #[test]
    fn check_equality() {
        let id = thread_rng().gen_range(0..i64::MAX);

        let cart_id1 = CartId::try_from(id).unwrap();
        let cart_id2 = CartId::try_from(id).unwrap();

        assert_eq!(cart_id1, cart_id1);
        assert_eq!(cart_id1.to_i64(), cart_id2.to_i64())
    }

    #[test]
    fn wrong_id_value() {
        let id = thread_rng().gen_range(i64::MIN..0);

        let cart_id = CartId::try_from(id);

        assert_eq!(cart_id.unwrap_err(), CartError::IdGenerationError);
    }
}
