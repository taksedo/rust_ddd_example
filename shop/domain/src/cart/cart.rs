use std::collections::HashMap;

use common::types::{
    base::{DomainEntity, DomainEntityTrait, Version, AM},
    common::Count,
};
use derive_getters::Getters;
use serde_derive::{Deserialize, Serialize};
use smart_default::SmartDefault;
use time::OffsetDateTime;

use crate::{
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

#[derive(Debug, Clone, PartialEq, SmartDefault, Serialize, Deserialize, Getters)]
pub struct Cart {
    #[getter(skip)]
    pub(crate) entity_params: DomainEntity<CartId, CartEventEnum>,
    #[default(Default::default())]
    pub(crate) for_customer: CustomerId,
    #[default(_code = "OffsetDateTime::now_utc()")]
    pub(crate) created: OffsetDateTime,
    pub(crate) meals: HashMap<MealId, Count>,
}

impl Cart {
    pub fn create(id_generator: AM<dyn CartIdGenerator>, for_customer: CustomerId) -> Self {
        let mut cart = Self {
            entity_params: DomainEntity {
                id: id_generator.lock().unwrap().generate(),
                version: Version::default(),
                events: vec![],
            },
            for_customer,
            created: OffsetDateTime::now_utc(),
            meals: HashMap::new(),
        };
        cart.add_event(CartCreatedDomainEvent::new(*cart.id()).into());
        cart
    }

    pub fn create_new_meal(&mut self, meal_id: &MealId) {
        self.meals.insert(*meal_id, Count::one());
        self.add_event(MealAddedToCartDomainEvent::new(*self.id(), *meal_id).into());
    }

    pub fn update_existing_meal(&mut self, meal_id: &MealId, count: Count) {
        count
            .increment()
            .map(|increment_count| {
                if let Some(x) = self.meals.get_mut(meal_id) {
                    *x = increment_count
                }
            })
            .expect("You have too much the same meals in you cart")
    }
    pub fn add_meal(&mut self, meal: Meal) {
        let meal_id = meal.id();
        let count_of_currently_meals_in_cart = self.meals.get(meal_id);
        if let Some(unwrapped_count) = count_of_currently_meals_in_cart {
            self.update_existing_meal(meal_id, *unwrapped_count)
        } else {
            self.create_new_meal(meal_id)
        }
    }

    pub fn remove_meals(&mut self, meal_id: &MealId) {
        if self.meals.remove(meal_id).is_some() {
            self.add_event(MealRemovedFromCartDomainEvent::new(*self.id(), *meal_id).into())
        }
    }

    pub fn id(&self) -> &CartId {
        self.entity_params.id()
    }

    pub fn version(&self) -> &Version {
        self.entity_params.version()
    }

    pub(self) fn add_event(&mut self, event: CartEventEnum) {
        self.entity_params.add_event(event)
    }

    pub fn pop_events(&mut self) -> Vec<CartEventEnum> {
        self.entity_params.pop_events()
    }
}

#[derive(Debug, PartialEq)]
pub enum CartError {
    IdGenerationError,
}

#[cfg(test)]
mod tests {
    use std::mem::discriminant;

    use common::{test_fixtures::rnd_count, types::base::AMW};

    use super::*;
    use crate::test_fixtures::{rnd_cart, rnd_cart_id, rnd_customer_id, rnd_meal};

    #[test]
    fn create_cart_success() {
        let customer_id = rnd_customer_id();
        let id_generator = AMW::new(TestCartIdGenerator::default());
        let mut cart = Cart::create(id_generator.clone(), customer_id);

        let id = id_generator.lock().unwrap().id;
        assert_eq!(cart.id(), &id);
        assert_eq!(cart.for_customer(), &customer_id);
        assert!(cart.meals().is_empty());
        assert!(cart.created() < &OffsetDateTime::now_utc());
        assert!(cart.pop_events().iter().all(|event| discriminant(event)
            == discriminant(&CartCreatedDomainEvent::new(rnd_cart_id()).into())));
    }

    #[test]
    fn add_meal_no_meal_in_cart_success() {
        let mut cart = rnd_cart();
        let meal = rnd_meal();

        cart.add_meal(meal.clone());
        assert!(cart
            .pop_events()
            .iter()
            .all(|event| { matches!(event, CartEventEnum::MealAddedToCartDomainEvent(_)) }));
        assert!(cart.meals().iter().all(|item| {
            let (&item_meal_id, &item_count) = item;
            (item_meal_id == *meal.id()) && (item_count == Count::one())
        }))
    }

    #[test]
    fn add_meal_has_meals_in_cart_success() {
        let meal = rnd_meal();
        let count = Count::try_from(2).unwrap();
        let mut cart = rnd_cart();
        cart.meals.insert(*meal.id(), count);

        cart.add_meal(meal.clone());
        assert!(cart.pop_events().iter().all(|event| {
            event == &MealAddedToCartDomainEvent::new(*cart.id(), *meal.id()).into()
        }));
        assert!(cart.meals().iter().all(|item| {
            let (&item_meal_id, &item_count) = item;
            (item_meal_id == *meal.id()) && (item_count == Count::try_from(3).unwrap())
        }))
    }

    #[test]
    fn remove_meal_cart_is_empty_success() {
        let meal = rnd_meal();
        let mut cart = rnd_cart();
        cart.remove_meals(meal.id());
        assert!(cart.pop_events().is_empty());
    }

    #[test]
    fn remove_meal_meal_not_in_cart() {
        let existing_meal = rnd_meal();
        let count = Count::try_from(12).unwrap();
        let non_existing_meal = rnd_meal();
        let meals = HashMap::from([(*existing_meal.id(), count)]);

        let mut cart = rnd_cart();

        cart.meals = meals.clone();

        cart.remove_meals(non_existing_meal.id());
        assert!(cart.pop_events().is_empty());
        assert!(cart.meals().iter().all(|item| {
            let (item_meal_id, &item_count) = item;
            meals.get_key_value(item_meal_id).unwrap() == (item_meal_id, &item_count)
        }));
    }

    #[test]
    fn remove_meal_meal_in_cart_success() {
        let meal_for_removing = rnd_meal();
        let removing_count = Count::try_from(12).unwrap();

        let meal = rnd_meal();
        let count = rnd_count();

        let meals = HashMap::from([
            (*meal_for_removing.id(), removing_count),
            (*meal.id(), count),
        ]);
        let mut cart = rnd_cart();
        cart.meals = meals.clone();

        cart.remove_meals(meal_for_removing.id());
        cart.pop_events().iter().all(|event| match event {
            CartEventEnum::MealRemovedFromCartDomainEvent(event_str) => {
                assert_eq!(event_str.meal_id, *meal_for_removing.id());
                assert_eq!(event_str.cart_id, *cart.id());
                true
            }
            _ => false,
        });
        assert!(cart.meals.iter().all(|item| {
            let (&item_meal_id, &item_count) = item;
            meals.get_key_value(&item_meal_id).unwrap() == (&item_meal_id, &item_count)
        }));
    }

    #[derive(Debug, SmartDefault)]
    struct TestCartIdGenerator {
        #[default(rnd_cart_id())]
        id: CartId,
    }

    impl CartIdGenerator for TestCartIdGenerator {
        fn generate(&mut self) -> CartId {
            self.id
        }
    }
}
