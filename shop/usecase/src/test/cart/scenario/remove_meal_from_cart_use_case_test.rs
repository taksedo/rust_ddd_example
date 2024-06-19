use std::sync::{Arc, Mutex};

use domain::test_fixtures::{rnd_cart, rnd_meal_id};

use crate::{
    main::cart::{
        remove_meal_from_cart::{RemoveMealFromCart, RemoveMealFromCartUseCaseError},
        scenarios::remove_meal_from_cart_use_case::RemoveMealFromCartUseCase,
    },
    test_fixtures::{MockCartExtractor, MockCartPersister},
};

#[test]
fn successfully_removed() {
    let cart = rnd_cart();
    let cart_persister = Arc::new(Mutex::new(MockCartPersister::default()));
    let cart_extractor = Arc::new(Mutex::new(MockCartExtractor::new(Some(cart.clone()), None)));

    let use_case = RemoveMealFromCartUseCase::new(cart_extractor.clone(), cart_persister.clone());
    let result = use_case.execute(cart.clone().get_for_customer(), &rnd_meal_id());

    cart_extractor
        .lock()
        .unwrap()
        .verify_invoked(&cart.get_for_customer());
    cart_persister
        .lock()
        .unwrap()
        .verify_invoked(Some(&cart), None, None, None);
    assert!(result.is_ok());
}

#[test]
fn cart_not_found() {
    let cart = rnd_cart();
    let cart_persister = Arc::new(Mutex::new(MockCartPersister::default()));
    let cart_extractor = Arc::new(Mutex::new(MockCartExtractor::default()));

    let use_case = RemoveMealFromCartUseCase::new(cart_extractor.clone(), cart_persister.clone());
    let result = use_case.execute(cart.clone().get_for_customer(), &rnd_meal_id());

    cart_extractor
        .lock()
        .unwrap()
        .verify_invoked(&cart.clone().get_for_customer());
    cart_persister.lock().unwrap().verify_empty();
    assert_eq!(
        result.unwrap_err(),
        RemoveMealFromCartUseCaseError::CartNotFound
    );
}
