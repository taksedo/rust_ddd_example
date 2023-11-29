use derive_new::new;
use domain::main::cart::value_objects::cart_id::{CartId, CartIdGenerator};
use domain::main::cart::value_objects::customer_id::CustomerId;
use std::sync::{Arc, Mutex};

use crate::main::cart::add_meal_to_cart::{AddMealToCart, AddMealToCartUseCaseError};
use domain::test_fixtures::rnd_meal;

use crate::main::cart::scenarios::add_meal_to_cart_use_case::AddMealToCartUseCase;
use crate::test_fixtures::{MockCartExtractor, MockCartPersister, MockMealExtractor};

#[test]
fn cart_doesnt_exist_successfully_added() {
    let meal = rnd_meal();
    let cart_persister = Arc::new(Mutex::new(MockCartPersister::default()));
    let cart_extractor = Arc::new(Mutex::new(MockCartExtractor::default()));
    let meal_extractor = Arc::new(Mutex::new(MockMealExtractor::new()));
    let id_generator = Arc::new(Mutex::new(TestCartIdGenerator::default()));

    let mut use_case = AddMealToCartUseCase::new(
        Arc::clone(&cart_extractor) as _,
        Arc::clone(&id_generator) as _,
        Arc::clone(&meal_extractor) as _,
        Arc::clone(&cart_persister) as _,
    );

    let result = use_case.execute(CustomerId::new(), meal.entity_params.id);

    meal_extractor
        .lock()
        .unwrap()
        .verify_invoked_get_by_id(&meal.entity_params.id);
    cart_persister.lock().unwrap().verify_empty();
    cart_extractor.lock().unwrap().verify_empty();
    assert_eq!(result, Err(AddMealToCartUseCaseError::MealNotFound))
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
