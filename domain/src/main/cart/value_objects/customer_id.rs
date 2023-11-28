use common::types::main::base::value_object::ValueObject;
use derive_new::new;
use serde_derive::{Deserialize, Serialize};

#[derive(new, Debug, Clone, Deserialize, Serialize, PartialEq, Default, Eq, Hash)]
pub struct CustomerId {
    value: String,
}

impl CustomerId {
    pub fn to_string(&self) -> String {
        self.value.to_owned()
    }
}

impl ValueObject for CustomerId {}
