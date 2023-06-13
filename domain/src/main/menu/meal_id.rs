use bigdecimal::ToPrimitive;
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(new, Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Default, Eq, Hash)]
pub struct MealId {
    pub value: i64,
}

impl MealId {
    pub fn to_i64(self) -> i64 {
        self.value.to_i64().unwrap()
    }
}

pub trait MealIdGenerator: Debug + Send {
    fn generate(&mut self) -> MealId;
}
