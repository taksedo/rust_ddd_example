use std::fmt::Debug;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Default, Eq, Hash)]
#[non_exhaustive]
pub struct CartId {}

pub trait CartIdGenerator: Debug + Send {
    fn generate(&mut self) -> CartId;
}
