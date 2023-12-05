use std::mem::{discriminant, Discriminant};
use std::sync::{Arc, Mutex};

use common::events::main::domain_event_listener::DomainEventListener;
use derive_new::new;
use tracing::info;

use domain::main::order::customer_order_events::{ShopOrderCreatedDomainEvent, ShopOrderEventEnum};

use crate::main::cart::access::cart_extractor::CartExtractor;
use crate::main::cart::access::cart_remover::CartRemover;

#[derive(new, Debug)]
pub struct RemoveCartAfterCheckoutRule {
    cart_extractor: Arc<Mutex<dyn CartExtractor>>,
    cart_remover: Arc<Mutex<dyn CartRemover>>,
}

impl DomainEventListener<ShopOrderEventEnum> for RemoveCartAfterCheckoutRule {
    fn event_type(&self) -> Discriminant<ShopOrderEventEnum> {
        let event: ShopOrderEventEnum = ShopOrderCreatedDomainEvent::default().into();
        discriminant(&event)
    }

    fn handle(&mut self, event: &ShopOrderEventEnum) {
        let event_struct: ShopOrderCreatedDomainEvent =
            event.clone().try_into().expect("Wrong type of event");

        let result = &self
            .cart_extractor
            .lock()
            .unwrap()
            .get_cart(event_struct.clone().for_customer);

        if result.is_none() {
            let _ = tracing_subscriber::fmt::try_init();
            info!(
                "Cart for customer #{} is already removed",
                event_struct.for_customer
            )
        } else {
            self.cart_remover
                .lock()
                .unwrap()
                .delete_cart(result.clone().unwrap())
        }
    }

    fn get_events(&self) -> &Vec<ShopOrderEventEnum> {
        todo!()
    }
}
