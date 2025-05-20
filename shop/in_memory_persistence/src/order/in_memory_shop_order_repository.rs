use std::collections::BTreeMap;

use common::{events::DomainEventPublisher, types::base::AM};
use derivative::Derivative;
use derive_new::new;
use domain::{
    cart::value_objects::customer_id::CustomerId,
    order::{
        customer_order_events::ShopOrderEventEnum, shop_order::ShopOrder,
        value_objects::shop_order_id::ShopOrderId,
    },
};
use usecase::order::access::{
    shop_order_extractor::ShopOrderExtractor, shop_order_persister::ShopOrderPersister,
};

#[derive(new, Clone, Derivative, Debug)]
pub struct InMemoryShopOrderRepository {
    event_publisher: AM<dyn DomainEventPublisher<ShopOrderEventEnum>>,
    #[new(value = "BTreeMap::new()")]
    pub storage: BTreeMap<ShopOrderId, ShopOrder>,
}

#[async_trait::async_trait]
impl ShopOrderPersister for InMemoryShopOrderRepository {
    async fn save(&mut self, mut order: ShopOrder) {
        self.event_publisher
            .lock()
            .await
            .publish(&order.pop_events())
            .await;
        self.storage.insert(*order.id(), order);
    }
}

impl ShopOrderExtractor for InMemoryShopOrderRepository {
    fn get_by_id(&mut self, order_id: &ShopOrderId) -> Option<ShopOrder> {
        self.storage.get(order_id).map(|order| order.to_owned())
    }

    fn get_last_order(&mut self, for_customer: &CustomerId) -> Option<ShopOrder> {
        self.storage
            .values()
            .filter(|order| order.for_customer() == for_customer)
            .collect::<Vec<_>>()
            .into_iter()
            .max_by(|o1, o2| o1.created().cmp(o2.created()))
            .cloned()
    }

    fn get_all(&mut self, start_id: &ShopOrderId, limit: usize) -> Vec<ShopOrder> {
        self.storage
            .range(start_id..)
            .take(limit)
            .map(|(_, order)| order.to_owned())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use common::types::base::AMTrait;
    use domain::{order::customer_order_events::ShopOrderCompletedDomainEvent, test_fixtures::*};
    use tokio::test;

    use super::*;
    use crate::test_fixtures::{TestEventPublisher, order_with_events};

    #[test]
    async fn saving_order_order_doesnt_exist() {
        let event_publisher = AM::new_am(TestEventPublisher::new());
        let mut repository = InMemoryShopOrderRepository::new(event_publisher.clone());
        let order = order_with_events();

        repository.save(order.clone()).await;

        let stored_order = repository.storage.get(order.id()).unwrap();
        assert_eq!(stored_order, &order);
        assert_eq!(event_publisher.lock().await.storage.len(), 1);

        let binding = event_publisher.lock().await;
        let event: &ShopOrderEventEnum = binding.storage.first().unwrap();
        let event_struct = TryInto::<ShopOrderCompletedDomainEvent>::try_into(event.clone());
        assert!(event_struct.is_ok());

        assert_eq!(&event_struct.unwrap().order_id, order.id());
    }

    #[test]
    async fn saving_order_order_exist() {
        let updated_order = order_with_events();
        let id = updated_order.id();

        let existing_order = rnd_order_with_id(*id);

        let event_publisher = AM::new_am(TestEventPublisher::new());
        let mut repository = InMemoryShopOrderRepository::new(event_publisher.clone());

        repository
            .storage
            .insert(*existing_order.id(), existing_order);

        repository.save(updated_order.clone()).await;

        let binding = event_publisher.lock().await;
        let event: &ShopOrderEventEnum = binding.storage.first().unwrap();
        let event_struct = TryInto::<ShopOrderCompletedDomainEvent>::try_into(event.clone());

        assert!(event_struct.is_ok());

        assert_eq!(&event_struct.unwrap().order_id, updated_order.id());
    }

    #[test]
    async fn get_by_id_order_exist() {
        let existing_order = rnd_order(Default::default());

        let event_publisher = AM::new_am(TestEventPublisher::new());
        let mut repository = InMemoryShopOrderRepository::new(event_publisher.clone());

        let id = existing_order.id();

        repository.storage.insert(*id, existing_order.clone());

        let order = repository.get_by_id(id);
        assert_eq!(order.unwrap(), existing_order);
    }

    #[test]
    async fn get_by_id_order_doesnt_exist() {
        let event_publisher = AM::new_am(TestEventPublisher::new());
        let mut repository = InMemoryShopOrderRepository::new(event_publisher.clone());
        let order = repository.get_by_id(&rnd_order_id());
        assert!(order.is_none());
    }

    #[test]
    async fn get_last_doesnt_exist() {
        let event_publisher = AM::new_am(TestEventPublisher::new());
        let mut repository = InMemoryShopOrderRepository::new(event_publisher.clone());
        let order = repository.get_last_order(&rnd_customer_id());
        assert!(order.is_none());
    }

    #[test]
    async fn get_last_success() {
        let customer_id = rnd_customer_id();
        let first_order = rnd_order_with_customer_id(customer_id);
        let last_order = rnd_order_with_customer_id(customer_id);
        let one_more_order = rnd_order_with_customer_id(rnd_customer_id());

        let event_publisher = AM::new_am(TestEventPublisher::new());
        let mut repository = InMemoryShopOrderRepository::new(event_publisher.clone());

        repository.save(first_order).await;
        repository.save(last_order.clone()).await;
        repository.save(one_more_order).await;

        let order = repository.get_last_order(&customer_id);
        assert_eq!(order.unwrap(), last_order);
    }

    #[test]
    async fn get_all_storage_is_empty() {
        let order_id = rnd_order_id();
        let event_publisher = AM::new_am(TestEventPublisher::new());
        let mut repository = InMemoryShopOrderRepository::new(event_publisher.clone());
        let order = repository.get_all(&order_id, 100);
        assert!(order.is_empty());
    }

    #[test]
    async fn get_all_limit_is_less_than_collection() {
        let limit = 10;
        let collection_size = 20;

        let event_publisher = AM::new_am(TestEventPublisher::new());
        let mut repository = InMemoryShopOrderRepository::new(event_publisher.clone());

        for i in 0..collection_size {
            let order = rnd_order_with_id(ShopOrderId::try_from(i).unwrap());
            repository.storage.insert(*order.id(), order);
        }

        let result = repository.get_all(&ShopOrderId::try_from(3).unwrap(), limit);

        assert_eq!(result.len(), limit);
        assert_eq!(result.first().unwrap().id().to_i64(), 3);
        assert_eq!(result.last().unwrap().id().to_i64(), 12);
    }

    #[test]
    async fn get_all_limit_is_bigger_than_collection() {
        let limit = 10;
        let collection_size = 5;

        let event_publisher = AM::new_am(TestEventPublisher::new());
        let mut repository = InMemoryShopOrderRepository::new(event_publisher.clone());

        for i in 0..collection_size {
            let order = rnd_order_with_id(ShopOrderId::try_from(i).unwrap());
            repository.storage.insert(*order.id(), order);
        }

        let result = repository.get_all(&ShopOrderId::try_from(0).unwrap(), limit);

        assert_eq!(result.len(), collection_size as usize);
        assert_eq!(result.first().unwrap().id().to_i64(), 0);
        assert_eq!(result.last().unwrap().id().to_i64(), 4);
    }
}
