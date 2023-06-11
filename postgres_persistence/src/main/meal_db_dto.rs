use bigdecimal::BigDecimal;
use common_types::main::base::domain_entity::{DomainEntity, Version};
use diesel::prelude::*;
use domain::main::menu::meal::Meal;
use domain::main::menu::meal_description::MealDescription;
use domain::main::menu::meal_id::MealId;
use domain::main::menu::meal_name::MealName;
use domain::main::menu::price::Price;
use serde::*;

#[derive(
    Insertable, Queryable, QueryableByName, Selectable, Serialize, Deserialize, Clone, Debug,
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
            id: value.domain_entity_field.id.to_i64(),
            name: value.name.to_string(),
            description: Some(value.description.to_string()),
            price: value.price.to_bigdecimal(),
            removed: value.removed,
            version: value.domain_entity_field.version.to_i64(),
        }
    }
}

impl From<MealDbDto> for Meal {
    fn from(value: MealDbDto) -> Self {
        Self {
            domain_entity_field: DomainEntity {
                id: MealId::new(value.id),
                version: Version::from(value.version),
                events: vec![],
            },
            name: MealName::from(value.name).unwrap(),
            description: MealDescription::from(value.description.unwrap()).unwrap(),
            price: Price::from(value.price).unwrap(),
            removed: value.removed,
        }
    }
}
