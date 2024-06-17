use std::sync::{Arc, Mutex};

use derive_new::new;
use domain::{
    main::cart::value_objects::cart_id::{CartId, CartIdGenerator},
    test_fixtures::{rnd_cart, rnd_customer_id, rnd_meal},
};

use crate::{
    main::cart::{
        add_meal_to_cart::{AddMealToCart, AddMealToCartUseCaseError},
        scenarios::add_meal_to_cart_use_case::AddMealToCartUseCase,
    },
    test_fixtures::{MockCartExtractor, MockCartPersister, MockMealExtractor},
};

#[test]
fn cart_doesnt_exist_successfully_added() {
    let meal = rnd_meal();
    let cart_persister = Arc::new(Mutex::new(MockCartPersister::default()));
    let cart_extractor = Arc::new(Mutex::new(MockCartExtractor::default()));
    let meal_extractor = Arc::new(Mutex::new(MockMealExtractor::new()));
    meal_extractor.lock().unwrap().meal = Some(meal.clone());
    let id_generator = Arc::new(Mutex::new(TestCartIdGenerator::default()));

    let mut use_case = AddMealToCartUseCase::new(
        cart_extractor.clone(),
        id_generator.clone(),
        meal_extractor.clone(),
        cart_persister.clone(),
    );

    let customer_id = rnd_customer_id();
    let result = use_case.execute(customer_id, meal.get_id());

    meal_extractor
        .lock()
        .unwrap()
        .verify_invoked_get_by_id(&meal.get_id());
    cart_persister.lock().unwrap().verify_invoked(
        None,
        Some(&id_generator.lock().unwrap().id),
        Some(meal.get_id()),
        Some(&customer_id),
    );
    assert!(result.is_ok())
}

#[test]
fn cart_exists_successfully_added() {
    let meal = rnd_meal();
    let customer_id = rnd_customer_id();
    let mut existing_cart = rnd_cart();
    existing_cart.for_customer = customer_id;

    let cart_persister = Arc::new(Mutex::new(MockCartPersister::default()));
    let meal_extractor = Arc::new(Mutex::new(MockMealExtractor::default()));
    meal_extractor.lock().unwrap().meal = Some(meal.clone());
    let cart_extractor = Arc::new(Mutex::new(MockCartExtractor::default()));
    cart_extractor.lock().unwrap().cart = Some(existing_cart.to_owned());

    let id_generator = Arc::new(Mutex::new(TestCartIdGenerator::default()));

    let mut use_case = AddMealToCartUseCase::new(
        cart_extractor.clone(),
        id_generator.clone(),
        meal_extractor.clone(),
        cart_persister.clone(),
    );

    let result = use_case.execute(customer_id, meal.clone().get_id());
    assert!(result.is_ok());

    meal_extractor
        .lock()
        .unwrap()
        .verify_invoked_get_by_id(&meal.get_id());

    let existing_cart = cart_persister.lock().unwrap().cart.clone().unwrap();

    cart_extractor.lock().unwrap().cart = Some(existing_cart.clone());

    cart_persister.lock().unwrap().verify_invoked(
        Some(&existing_cart),
        None,
        Some(&meal.get_id()),
        None,
    );
    cart_extractor.lock().unwrap().verify_invoked(&customer_id);
}

#[test]
fn mel_not_found() {
    let meal = rnd_meal();
    let cart_persister = Arc::new(Mutex::new(MockCartPersister::default()));
    let cart_extractor = Arc::new(Mutex::new(MockCartExtractor::default()));
    let meal_extractor = Arc::new(Mutex::new(MockMealExtractor::default()));
    let id_generator = Arc::new(Mutex::new(TestCartIdGenerator::default()));

    let mut use_case = AddMealToCartUseCase::new(
        cart_extractor.clone(),
        id_generator.clone(),
        meal_extractor.clone(),
        cart_persister.clone(),
    );

    let result = use_case.execute(rnd_customer_id(), meal.get_id());

    meal_extractor
        .lock()
        .unwrap()
        .verify_invoked_get_by_id(&meal.get_id());
    cart_persister.lock().unwrap().verify_empty();
    cart_extractor.lock().unwrap().verify_empty();
    assert_eq!(result.unwrap_err(), AddMealToCartUseCaseError::MealNotFound);
}

#[derive(new, Debug, Default)]
struct TestCartIdGenerator {
    id: CartId,
}

impl CartIdGenerator for TestCartIdGenerator {
    fn generate(&mut self) -> CartId {
        self.id
    }
}
