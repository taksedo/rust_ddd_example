#![allow(special_module_name)]

pub mod cart;
pub mod menu;
pub mod order;
#[cfg(test)]
#[path = "../../../test_fixtures/persistence.rs"]
pub mod test_fixtures;

#[cfg(test)]
#[path = "../../../test_fixtures/common.rs"]
pub mod common_test_fixtures;
#[cfg(test)]
#[path = "../../../test_fixtures/domain.rs"]
pub mod domain_test_fixtures;

#[cfg(test)]
#[path = "../../../test_fixtures/usecase.rs"]
pub mod usecase_test_fixtures;
