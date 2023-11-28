use bigdecimal::BigDecimal;
use common::types::main::base::domain_entity::{DomainEntity, Version};
use diesel::prelude::*;
use serde::*;

use domain::main::menu::meal::Meal;
use domain::main::menu::value_objects::meal_description::MealDescription;
use domain::main::menu::value_objects::meal_id::MealId;
use domain::main::menu::value_objects::meal_name::MealName;
use domain::main::menu::value_objects::price::Price;

#[derive(
    Insertable,
    Identifiable,
    Queryable,
    QueryableByName,
    Selectable,
    AsChangeset,
    Serialize,
    Deserialize,
    Clone,
    Debug,
)]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::main::schema::shop::meal)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MealDbDto {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub removed: bool,
    pub version: i64,
}

impl From<Meal> for MealDbDto {
    fn from(value: Meal) -> Self {
        Self {
            id: value.entity_params.id.to_i64(),
            name: value.name.to_string(),
            description: Some(value.description.to_string()),
            price: value.price.to_bigdecimal(),
            removed: value.removed,
            version: value.entity_params.version.to_i64(),
        }
    }
}

impl From<MealDbDto> for Meal {
    fn from(value: MealDbDto) -> Self {
        Self {
            entity_params: DomainEntity {
                id: MealId::try_from(value.id).unwrap(),
                version: Version::from(value.version),
                events: vec![],
            },
            name: MealName::try_from(value.name.as_str()).unwrap(),
            description: MealDescription::try_from(value.description.unwrap().as_str()).unwrap(),
            price: Price::try_from(value.price).unwrap(),
            removed: value.removed,
        }
    }
}
