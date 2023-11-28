use std::fmt::Debug;

use derive_new::new;
use serde_derive::{Deserialize, Serialize};

#[derive(new, Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Default, Eq, Hash)]
pub struct CartId {
    value: i64,
}

impl CartId {
    pub fn to_long_value(&self) -> i64 {
        self.value
    }
}

pub trait CartIdGenerator: Debug + Send {
    fn generate(&mut self) -> CartId;
}
