use std::{
    collections::HashMap,
    mem::discriminant,
    sync::{Arc, Mutex},
};

use common::types::{common::count::Count, test_fixtures::rnd_count};
use smart_default::SmartDefault;
use time::OffsetDateTime;

use crate::{
    main::cart::{
        cart::Cart,
        cart_events::{CartCreatedDomainEvent, CartEventEnum, MealAddedToCartDomainEvent},
        value_objects::cart_id::{CartId, CartIdGenerator},
    },
    test_fixtures::{rnd_cart, rnd_cart_id, rnd_customer_id, rnd_meal},
};

#[test]
fn create_cart_success() {
    let customer_id = rnd_customer_id();
    let id_generator = Arc::new(Mutex::new(TestCartIdGenerator::default()));
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
    assert!(cart
        .pop_events()
        .iter()
        .all(|event| { event == &MealAddedToCartDomainEvent::new(*cart.id(), *meal.id()).into() }));
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
        meals.get_key_value(&item_meal_id).unwrap() == (&item_meal_id, &item_count)
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
