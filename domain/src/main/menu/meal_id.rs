use core::any::Any;
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(new, Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Default, Eq, Hash)]
pub struct MealId {
    pub value: i64,
}

impl MealId {
    fn _to_long_value(self) -> i64 {
        self.value
    }
}

pub trait MealIdGenerator: Debug + Any {
    fn generate(&self) -> MealId;
}
