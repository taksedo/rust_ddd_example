use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use common::events::main::domain_event_publisher::DomainEventPublisher;
use common::types::main::base::domain_entity::DomainEntityTrait;
use derivative::Derivative;
use derive_new::new;

use domain::main::cart::cart::Cart;
use domain::main::cart::cart_events::CartEventEnum;
use domain::main::cart::value_objects::customer_id::CustomerId;
use usecase::main::cart::access::cart_extractor::CartExtractor;
use usecase::main::cart::access::cart_persister::CartPersister;
use usecase::main::cart::access::cart_remover::CartRemover;

#[derive(new, Clone, Derivative, Debug)]
pub struct InMemoryCartRepository {
    event_publisher: Arc<Mutex<dyn DomainEventPublisher<CartEventEnum>>>,
    #[new(value = "HashMap::new()")]
    pub storage: HashMap<CustomerId, Cart>,
}

impl CartExtractor for InMemoryCartRepository {
    fn get_cart(&mut self, for_customer: CustomerId) -> Option<Cart> {
        Some(self.storage.get(&for_customer)?.clone())
    }
}

impl CartPersister for InMemoryCartRepository {
    fn save(&mut self, mut cart: Cart) {
        dbg!(&cart);
        let popped_events = cart.entity_param.pop_events();
        dbg!(&popped_events);
        self.event_publisher.lock().unwrap().publish(&popped_events);
        self.storage.insert(cart.for_customer, cart);
    }
}

impl CartRemover for InMemoryCartRepository {
    fn delete_cart(&mut self, cart: Cart) {
        self.storage.remove(&cart.for_customer);
    }
}
