// use std::mem::{discriminant, Discriminant};
//
// use derive_new::new;
// use tiny_kafka::producer::{KafkaProducer, Message};
//
// use common_events::main::domain_event_listener::DomainEventListener;
// use domain::main::menu::meal_events::DomainEventEnum;
//
// #[derive(new, Debug, Default, Clone)]
// struct KafkaEventListenerImpl {
//     pub producer: KafkaProducer,
//     pub event: DomainEventEnum,
// }
//
// impl DomainEventListener<DomainEventEnum> for KafkaEventListenerImpl {
//     fn event_type(&self) -> Discriminant<DomainEventEnum> {
//         let event = &self.event;
//         discriminant(&event)
//     }
//
//     fn handle(&mut self, event: &DomainEventEnum) {
//         let msg = Message::new("key1", "");
//     }
//
//     fn get_events(&self) -> &Vec<DomainEventEnum> {
//         todo!()
//     }
// }
