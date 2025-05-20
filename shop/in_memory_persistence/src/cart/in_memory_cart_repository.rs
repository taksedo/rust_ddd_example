use std::collections::HashMap;

use async_trait::async_trait;
use common::{events::DomainEventPublisher, types::base::AM};
use derivative::Derivative;
use derive_new::new;
use domain::cart::{
    cart::Cart, cart_events::CartEventEnum, value_objects::customer_id::CustomerId,
};
use usecase::cart::access::{
    cart_extractor::CartExtractor, cart_persister::CartPersister, cart_remover::CartRemover,
};

#[derive(new, Clone, Derivative, Debug)]
pub struct InMemoryCartRepository {
    event_publisher: AM<dyn DomainEventPublisher<CartEventEnum>>,
    #[new(value = "HashMap::new()")]
    pub storage: HashMap<CustomerId, Cart>,
}

#[async_trait]
impl CartExtractor for InMemoryCartRepository {
    async fn get_cart(&mut self, for_customer: &CustomerId) -> Option<Cart> {
        Some(self.storage.get(for_customer)?.clone())
    }
}

#[async_trait]
impl CartPersister for InMemoryCartRepository {
    async fn save(&mut self, mut cart: Cart) {
        dbg!(&cart);
        let popped_events = cart.pop_events();
        dbg!(&popped_events);
        self.event_publisher
            .lock()
            .await
            .publish(&popped_events)
            .await;
        self.storage.insert(*cart.for_customer(), cart);
    }
}

impl CartRemover for InMemoryCartRepository {
    fn delete_cart(&mut self, cart: Cart) {
        self.storage.remove(cart.for_customer());
    }
}

#[cfg(test)]
mod tests {
    use common::types::base::AMTrait;
    use domain::{cart::cart_events::MealAddedToCartDomainEvent, test_fixtures::*};

    use super::*;
    use crate::test_fixtures::*;

    #[tokio::test]
    async fn saving_cart_cart_doesnt_exist() {
        let event_publisher = AM::new_am(TestEventPublisher::new());
        let mut repository = InMemoryCartRepository::new(event_publisher.clone());
        let cart = cart_with_events();

        repository.save(cart.clone()).await;

        let stored_cart = repository.storage.get(cart.for_customer()).unwrap();
        assert_eq!(stored_cart, &cart);
        assert_eq!(event_publisher.lock().await.storage.len(), 1);

        let binding = event_publisher.lock().await;
        let event: &CartEventEnum = binding.storage.first().unwrap();
        let event_struct: MealAddedToCartDomainEvent = event.clone().try_into().unwrap();
        assert_eq!(event_struct.cart_id, *cart.id());
    }

    #[tokio::test]
    async fn saving_cart_cart_exists() {
        let customer_id = rnd_customer_id();
        let existing_cart = rnd_cart_with_customer_id(customer_id);

        let event_publisher = AM::new_am(TestEventPublisher::new());
        let mut repository = InMemoryCartRepository::new(event_publisher.clone());
        repository.storage.insert(customer_id, existing_cart);

        let updated_cart = cart_with_events();
        repository.save(updated_cart.clone()).await;
        repository.storage.insert(customer_id, updated_cart.clone());

        let binding = event_publisher.lock().await;
        let event: &CartEventEnum = binding.storage.first().unwrap();
        let event_struct: Result<MealAddedToCartDomainEvent, _> = event.clone().try_into();
        assert!(event_struct.is_ok());
        assert_eq!(event_struct.unwrap().cart_id, *updated_cart.id());
    }

    #[tokio::test]
    async fn get_by_id_cart_exists() {
        let customer_id = rnd_customer_id();
        let existing_cart = rnd_cart_with_customer_id(customer_id);

        let event_publisher = AM::new_am(TestEventPublisher::new());
        let mut repository = InMemoryCartRepository::new(event_publisher.clone());
        repository
            .storage
            .insert(customer_id, existing_cart.clone());

        let cart = repository.get_cart(&customer_id).await;

        assert!(cart.is_some());

        let cart = cart.unwrap();
        assert_eq!(cart, existing_cart);
        assert_eq!(cart.id(), existing_cart.id());
        assert_eq!(cart.for_customer(), existing_cart.for_customer());
        assert_eq!(cart.created(), existing_cart.created());
        assert_eq!(cart.meals(), existing_cart.meals());
    }

    #[tokio::test]
    async fn get_by_id_cart_doesnt_exist() {
        let event_publisher = AM::new_am(TestEventPublisher::new());
        let mut repository = InMemoryCartRepository::new(event_publisher.clone());
        let cart = repository.get_cart(&rnd_customer_id()).await;

        assert!(cart.is_none());
    }

    #[test]
    fn delete_cart_cart_exists() {
        let existing_cart = rnd_cart();
        let event_publisher = AM::new_am(TestEventPublisher::new());
        let mut repository = InMemoryCartRepository::new(event_publisher.clone());
        repository
            .storage
            .insert(*existing_cart.for_customer(), existing_cart.clone());

        repository.delete_cart(existing_cart);
        assert!(repository.storage.is_empty());
    }

    #[test]
    fn copy_cart_test() {
        let src = cart_with_events();
        assert!(!src.meals().is_empty());

        let copy = src.clone();

        assert_eq!(src, copy);
        assert_eq!(src.id(), copy.id());
        assert_eq!(src.for_customer(), copy.for_customer());
        assert_eq!(src.created(), copy.created());
        assert_eq!(src.meals(), copy.meals());
    }
}
