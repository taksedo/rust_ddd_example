use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use common::events::main::domain_event_publisher::DomainEventPublisher;
use common::types::main::base::domain_entity::DomainEntityTrait;
use derivative::Derivative;
use derive_new::new;

use domain::main::cart::value_objects::customer_id::CustomerId;
use domain::main::order::customer_order_events::ShopOrderEventEnum;
use domain::main::order::shop_order::ShopOrder;
use domain::main::order::value_objects::shop_order_id::ShopOrderId;
use usecase::main::order::access::shop_order_extractor::ShopOrderExtractor;
use usecase::main::order::access::shop_order_persister::ShopOrderPersister;

#[derive(new, Clone, Derivative, Debug)]
pub struct InMemoryShopOrderRepository {
    event_publisher: Arc<Mutex<dyn DomainEventPublisher<ShopOrderEventEnum>>>,
    #[new(value = "BTreeMap::new()")]
    pub storage: BTreeMap<ShopOrderId, ShopOrder>,
}

impl ShopOrderPersister for InMemoryShopOrderRepository {
    fn save(&mut self, mut order: ShopOrder) {
        self.event_publisher
            .lock()
            .unwrap()
            .publish(&order.entity_params.pop_events());
        self.storage.insert(order.entity_params.id, order);
    }
}

impl ShopOrderExtractor for InMemoryShopOrderRepository {
    fn get_by_id(&mut self, order_id: ShopOrderId) -> Option<ShopOrder> {
        self.storage
            .get(&order_id)
            .map(|order| order.to_owned())
            .take()
    }

    fn get_last_order(&mut self, for_customer: CustomerId) -> Option<ShopOrder> {
        match self
            .storage
            .values()
            .filter(|order| order.for_customer == for_customer)
            .collect::<Vec<_>>()
            .into_iter()
            .max_by(|o1, o2| o1.created.cmp(&o2.created))
        {
            None => None,
            Some(result) => Some(result.clone()),
        }
    }

    fn get_all(&mut self, start_id: ShopOrderId, limit: usize) -> Vec<ShopOrder> {
        self.storage
            .range(start_id..)
            .take(limit)
            .map(|(_, order)| order.to_owned())
            .collect()
    }
}
