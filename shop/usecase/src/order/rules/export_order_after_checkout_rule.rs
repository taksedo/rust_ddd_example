use std::mem::{Discriminant, discriminant};

use common::{
    events::DomainEventListener,
    types::base::{AM, ArcMutexTrait},
};
use derive_new::new;
use domain::order::customer_order_events::{ShopOrderCreatedDomainEvent, ShopOrderEventEnum};

use crate::order::providers::order_exporter::OrderExporter;

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

        self.order_exporter.lock_un().export_order(
            event_struct.order_id,
            event_struct.for_customer,
            event_struct.total_price,
        );
    }

    fn get_events(&self) -> &Vec<ShopOrderEventEnum> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use common::types::base::{AM, ArcMutexTrait};
    use domain::test_fixtures::*;

    use super::*;
    use crate::test_fixtures::MockOrderExporter;

    #[test]
    fn order_has_been_exported() {
        let order_id = rnd_order_id();
        let customer_id = rnd_customer_id();
        let total_price = rnd_price();

        let exporter = AM::new_am(MockOrderExporter::default());
        let mut rule = ExportOrderAfterCheckoutRule::new(exporter.clone());

        let event: ShopOrderEventEnum =
            ShopOrderCreatedDomainEvent::new(order_id, customer_id, total_price.clone()).into();

        rule.handle(&event);

        exporter
            .lock_un()
            .verify_invoked(order_id, customer_id, total_price);
    }
}
