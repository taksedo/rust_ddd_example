use std::sync::{Arc, Mutex};

use domain::main::cart::cart_events::{CartEventEnum, MealAddedToCartDomainEvent};
use domain::test_fixtures::{rnd_cart, rnd_customer_id};
use usecase::main::cart::access::cart_persister::CartPersister;

use crate::main::cart::in_memory_cart_repository::InMemoryCartRepository;
use crate::test_fixtures::{cart_with_events, TestEventPublisher};

#[test]
fn saving_cart_cart_doesnt_exist() {
    let event_publisher = Arc::new(Mutex::new(TestEventPublisher::new()));
    let mut repository = InMemoryCartRepository::new(Arc::clone(&event_publisher) as _);
    let cart = cart_with_events();

    repository.save(cart.clone());

    let stored_cart = repository.storage.get(&cart.for_customer).unwrap();
    assert_eq!(stored_cart, &cart);
    assert_eq!(event_publisher.lock().unwrap().storage.len(), 1);

    let binding = event_publisher.lock().unwrap();
    let event: &CartEventEnum = binding.storage.first().unwrap();
    let event_struct: MealAddedToCartDomainEvent = event.clone().try_into().unwrap();
    assert_eq!(event_struct.cart_id, cart.entity_param.id);
}

#[test]
fn saving_cart_cart_exists() {
    let customer_id = rnd_customer_id();
    let mut existing_cart = rnd_cart();
    existing_cart.for_customer = customer_id;

    let event_publisher = Arc::new(Mutex::new(TestEventPublisher::new()));
    let mut repository = InMemoryCartRepository::new(Arc::clone(&event_publisher) as _);
    repository.storage.insert(customer_id, existing_cart);

    let updated_cart = cart_with_events();
    repository.save(updated_cart.clone());
    repository.storage.insert(customer_id, updated_cart.clone());

    let binding = event_publisher.lock().unwrap();
    let event: &CartEventEnum = binding.storage.first().unwrap();
    let event_struct: Result<MealAddedToCartDomainEvent, _> = event.clone().try_into();
    assert!(event_struct.is_ok());
    assert_eq!(event_struct.unwrap().cart_id, updated_cart.entity_param.id);
}
