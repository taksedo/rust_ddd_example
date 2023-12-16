use std::collections::HashMap;

use common::types::main::{
    base::domain_entity::{DomainEntity, Version},
    common::count::Count,
};
use time::OffsetDateTime;

use crate::main::{
    cart::{
        cart::Cart,
        value_objects::{cart_id::CartId, customer_id::CustomerId},
    },
    menu::value_objects::meal_id::MealId,
};

pub struct CartRestorer {}

impl CartRestorer {
    pub fn restore_cart(
        id: CartId,
        for_customer: CustomerId,
        created: OffsetDateTime,
        meals: HashMap<MealId, Count>,
        version: Version,
    ) -> Cart {
        Cart {
            entity_param: DomainEntity {
                id,
                version,
                events: vec![],
            },
            for_customer,
            created,
            meals,
        }
    }
}
