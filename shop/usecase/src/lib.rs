#![allow(special_module_name)]

pub mod cart;
pub mod menu;
pub mod order;

#[cfg(all(test, feature = "testing"))]
pub mod test_fixtures;
