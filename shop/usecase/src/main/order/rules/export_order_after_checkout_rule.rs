use std::mem::{discriminant, Discriminant};

use common::{events::domain_event_listener::DomainEventListener, types::base::generic_types::AM};
use derive_new::new;
use domain::main::order::customer_order_events::{ShopOrderCreatedDomainEvent, ShopOrderEventEnum};

use crate::main::order::providers::order_exporter::OrderExporter;

#[derive(new, Debug)]
pub struct ExportOrderAfterCheckoutRule<OExporter: OrderExporter> {
    pub order_exporter: AM<OExporter>,
}

impl<OExported: OrderExporter> DomainEventListener<ShopOrderEventEnum>
    for ExportOrderAfterCheckoutRule<OExported>
{
    fn event_type(&self) -> Discriminant<ShopOrderEventEnum> {
        let event: ShopOrderEventEnum = ShopOrderCreatedDomainEvent::default().into();
        discriminant(&event)
    }

    fn handle(&mut self, event: &ShopOrderEventEnum) {
        let event_struct: ShopOrderCreatedDomainEvent =
            event.clone().try_into().expect("Wrong type of event");

        self.order_exporter.lock().unwrap().export_order(
            event_struct.order_id,
            event_struct.for_customer,
            event_struct.total_price,
        );
    }

    fn get_events(&self) -> &Vec<ShopOrderEventEnum> {
        todo!()
    }
}
