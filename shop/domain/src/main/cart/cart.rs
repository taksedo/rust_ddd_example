use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use common::types::{
    base::domain_entity::{DomainEntity, DomainEntityTrait, Version},
    common::count::Count,
};
use serde_derive::{Deserialize, Serialize};
use smart_default::SmartDefault;
use time::OffsetDateTime;

use crate::main::{
    cart::{
        cart_events::{
            CartCreatedDomainEvent, CartEventEnum, MealAddedToCartDomainEvent,
            MealRemovedFromCartDomainEvent,
        },
        value_objects::{
            cart_id::{CartId, CartIdGenerator},
            customer_id::CustomerId,
        },
    },
    menu::{meal::Meal, value_objects::meal_id::MealId},
};

#[derive(Debug, Clone, PartialEq, SmartDefault, Serialize, Deserialize)]
pub struct Cart {
    pub entity_param: DomainEntity<CartId, CartEventEnum>,
    #[default(Default::default())]
    pub for_customer: CustomerId,
    #[default(_code = "OffsetDateTime::now_utc()")]
    pub created: OffsetDateTime,
    pub meals: HashMap<MealId, Count>,
}

impl Cart {
    pub fn create(id_generator: Arc<Mutex<dyn CartIdGenerator>>, for_customer: CustomerId) -> Self {
        let mut cart = Self {
            entity_param: DomainEntity {
                id: id_generator.lock().unwrap().generate(),
                version: Version::default(),
                events: vec![],
            },
            for_customer,
            created: OffsetDateTime::now_utc(),
            meals: HashMap::new(),
        };
        cart.entity_param
            .add_event(CartCreatedDomainEvent::new(cart.entity_param.id).into());
        cart
    }

    pub fn create_new_meal(&mut self, meal_id: MealId) {
        self.meals.insert(meal_id, Count::one());
        self.entity_param
            .add_event(MealAddedToCartDomainEvent::new(self.entity_param.id, meal_id).into());
    }

    pub fn update_existing_meal(&mut self, meal_id: MealId, count: Count) {
        count
            .increment()
            .map(|increment_count| {
                if let Some(x) = self.meals.get_mut(&meal_id) {
                    *x = increment_count
                }
            })
            .expect("You have too much the same meals in you cart")
    }
    pub fn add_meal(&mut self, meal: Meal) {
        let meal_id = meal.entity_params.id;
        let count_of_currently_meals_in_cart = self.meals.get(&meal_id);
        if let Some(unwrapped_count) = count_of_currently_meals_in_cart {
            self.update_existing_meal(meal_id, *unwrapped_count)
        } else {
            self.create_new_meal(meal_id)
        }
    }

    pub fn remove_meals(&mut self, meal_id: MealId) {
        if self.meals.remove(&meal_id).is_some() {
            self.entity_param.add_event(
                MealRemovedFromCartDomainEvent::new(self.entity_param.id, meal_id).into(),
            )
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CartError {
    IdGenerationError,
}
