#![allow(special_module_name)]

pub mod cart;
pub mod menu;
pub mod order;

#[cfg(any(test, feature = "testing"))]
pub mod test_fixtures;
