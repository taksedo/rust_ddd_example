use std::sync::{Arc, Mutex};

use domain::{
    main::order::{
        customer_order_events::{ShopOrderCompletedDomainEvent, ShopOrderEventEnum},
        value_objects::shop_order_id::ShopOrderId,
    },
    test_fixtures::{rnd_customer_id, rnd_order, rnd_order_id},
};
use usecase::main::order::access::{
    shop_order_extractor::ShopOrderExtractor, shop_order_persister::ShopOrderPersister,
};

use crate::{
    main::order::in_memory_shop_order_repository::InMemoryShopOrderRepository,
    test_fixtures::{order_with_events, TestEventPublisher},
};

#[test]
fn saving_order_order_doesnt_exist() {
    let event_publisher = Arc::new(Mutex::new(TestEventPublisher::new()));
    let mut repository = InMemoryShopOrderRepository::new(event_publisher.clone() as _);
    let order = order_with_events();

    repository.save(order.clone());

    let stored_order = repository.storage.get(&order.entity_params.id).unwrap();
    assert_eq!(stored_order, &order);
    assert_eq!(event_publisher.lock().unwrap().storage.len(), 1);

    let binding = event_publisher.lock().unwrap();
    let event: &ShopOrderEventEnum = binding.storage.first().unwrap();
    let event_struct = TryInto::<ShopOrderCompletedDomainEvent>::try_into(event.clone());
    assert!(event_struct.is_ok());

    assert_eq!(event_struct.unwrap().order_id, order.entity_params.id);
}

#[test]
fn saving_order_order_exist() {
    let updated_order = order_with_events();
    let id = updated_order.entity_params.id;

    let mut existing_order = rnd_order(Default::default());
    existing_order.entity_params.id = id;

    let event_publisher = Arc::new(Mutex::new(TestEventPublisher::new()));
    let mut repository = InMemoryShopOrderRepository::new(event_publisher.clone() as _);

    repository
        .storage
        .insert(existing_order.entity_params.id, existing_order);

    repository.save(updated_order.clone());

    let binding = event_publisher.lock().unwrap();
    let event: &ShopOrderEventEnum = binding.storage.first().unwrap();
    let event_struct = TryInto::<ShopOrderCompletedDomainEvent>::try_into(event.clone());

    assert!(event_struct.is_ok());

    assert_eq!(
        event_struct.unwrap().order_id,
        updated_order.entity_params.id
    );
}

#[test]
fn get_by_id_order_exist() {
    let existing_order = rnd_order(Default::default());

    let event_publisher = Arc::new(Mutex::new(TestEventPublisher::new()));
    let mut repository = InMemoryShopOrderRepository::new(event_publisher.clone() as _);

    let id = existing_order.entity_params.id;

    repository.storage.insert(id, existing_order.clone());

    let order = repository.get_by_id(id);
    assert_eq!(order.unwrap(), existing_order);
}

#[test]
fn get_by_id_order_doesnt_exist() {
    let event_publisher = Arc::new(Mutex::new(TestEventPublisher::new()));
    let mut repository = InMemoryShopOrderRepository::new(event_publisher.clone() as _);
    let order = repository.get_by_id(rnd_order_id());
    assert!(order.is_none());
}

#[test]
fn get_last_doesnt_exist() {
    let event_publisher = Arc::new(Mutex::new(TestEventPublisher::new()));
    let mut repository = InMemoryShopOrderRepository::new(event_publisher.clone() as _);
    let order = repository.get_last_order(rnd_customer_id());
    assert!(order.is_none());
}

#[test]
fn get_last_success() {
    let customer_id = rnd_customer_id();
    let mut first_order = rnd_order(Default::default());
    let mut last_order = rnd_order(Default::default());
    let one_more_order = rnd_order(Default::default());

    first_order.for_customer = customer_id;
    last_order.for_customer = customer_id;

    let event_publisher = Arc::new(Mutex::new(TestEventPublisher::new()));
    let mut repository = InMemoryShopOrderRepository::new(event_publisher.clone() as _);

    repository.save(first_order);
    repository.save(last_order.clone());
    repository.save(one_more_order);

    let order = repository.get_last_order(customer_id);
    assert_eq!(order.unwrap(), last_order);
}

#[test]
fn get_all_storage_is_empty() {
    let order_id = rnd_order_id();
    let event_publisher = Arc::new(Mutex::new(TestEventPublisher::new()));
    let mut repository = InMemoryShopOrderRepository::new(event_publisher.clone() as _);
    let order = repository.get_all(order_id, 100);
    assert!(order.is_empty());
}

#[test]
fn get_all_limit_is_less_than_collection() {
    let limit = 10;
    let collection_size = 20;

    let event_publisher = Arc::new(Mutex::new(TestEventPublisher::new()));
    let mut repository = InMemoryShopOrderRepository::new(event_publisher.clone() as _);

    for i in 0..collection_size {
        let mut order = rnd_order(Default::default());
        order.entity_params.id = ShopOrderId::try_from(i).unwrap();
        repository.storage.insert(order.entity_params.id, order);
    }

    let result = repository.get_all(ShopOrderId::try_from(3).unwrap(), limit);

    assert_eq!(result.len(), limit);
    assert_eq!(result.first().unwrap().entity_params.id.to_i64(), 3);
    assert_eq!(result.last().unwrap().entity_params.id.to_i64(), 12);
}

#[test]
fn get_all_limit_is_bigger_than_collection() {
    let limit = 10;
    let collection_size = 5;

    let event_publisher = Arc::new(Mutex::new(TestEventPublisher::new()));
    let mut repository = InMemoryShopOrderRepository::new(event_publisher.clone() as _);

    for i in 0..collection_size {
        let mut order = rnd_order(Default::default());
        order.entity_params.id = ShopOrderId::try_from(i).unwrap();
        repository.storage.insert(order.entity_params.id, order);
    }

    let result = repository.get_all(ShopOrderId::try_from(0).unwrap(), limit);

    assert_eq!(result.len(), collection_size as usize);
    assert_eq!(result.first().unwrap().entity_params.id.to_i64(), 0);
    assert_eq!(result.last().unwrap().entity_params.id.to_i64(), 4);
}
