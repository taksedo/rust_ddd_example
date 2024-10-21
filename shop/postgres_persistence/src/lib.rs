#![allow(special_module_name)]

pub mod database_start;
pub mod meal_db_dto;
pub mod postgres_meal_id_generator;
pub mod postgres_meal_repository;
pub mod schema;

#[path = "../../../test_fixtures/common.rs"]
#[cfg(test)]
pub mod common_test_fixtures;
#[path = "../../../test_fixtures/domain.rs"]
#[cfg(test)]
pub mod domain_test_fixtures;
