use common::types::main::base::value_object::ValueObject;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Default, Eq, Hash)]
#[non_exhaustive]
pub struct CustomerId {
    value: String,
}

impl CustomerId {
    pub fn to_string(&self) -> String {
        self.value.to_owned()
    }
}

impl ValueObject for CustomerId {}
