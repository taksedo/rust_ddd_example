use bigdecimal::BigDecimal;
use common::types::base::{DomainEntity, Version};
use diesel::prelude::*;
use domain::menu::{
    meal::Meal,
    value_objects::{
        meal_description::MealDescription, meal_id::MealId, meal_name::MealName, price::Price,
    },
};
use serde::*;

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
#[diesel(table_name = crate::schema::shop::meal)]
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
            id: value.id().to_i64(),
            name: value.name().to_string(),
            description: Some(value.description().to_string()),
            price: value.price().to_bigdecimal(),
            removed: *value.removed(),
            version: value.version().to_i64(),
        }
    }
}

impl From<MealDbDto> for Meal {
    fn from(value: MealDbDto) -> Self {
        Self::with_all_args(
            DomainEntity::new(
                MealId::try_from(value.id).unwrap(),
                Version::from(value.version),
            ),
            MealName::try_from(value.name.as_str()).unwrap(),
            MealDescription::try_from(value.description.unwrap().as_str()).unwrap(),
            Price::try_from(value.price).unwrap(),
            value.removed,
        )
    }
}
