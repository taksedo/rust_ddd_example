#![allow(dead_code)]

use core::any::Any;
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(new, Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Default, Eq, Hash)]
pub struct MealId {
    pub value: i64,
}

impl MealId {
    fn to_long_value(self) -> i64 {
        self.value
    }
}

pub trait MealIdGenerator: Debug + Any {
    fn generate(&self) -> MealId;
    fn as_any(&self) -> &dyn Any;
}

// impl<T: Any + Debug> MealIdGenerator for T {
//     fn generate(&self) -> MealId {
//         MealId::new(0)
//     }
//
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }

impl<T: PartialEq + Any> PartialEq<T> for dyn MealIdGenerator {
    fn eq(&self, other: &T) -> bool {
        if let Some(this) = self.as_any().downcast_ref::<T>() {
            this == other
        } else {
            false
        }
    }
}
