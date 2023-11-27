use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::mem::{discriminant, Discriminant};
use std::sync::{Arc, Mutex};

use derive_new::new;

use common::events::main::domain_event_listener::DomainEventListener;
use common::events::main::domain_event_publisher::DomainEventPublisher;

type VecOfDomainEventListenerType<Event> = Vec<Arc<Mutex<dyn DomainEventListener<Event>>>>;

#[derive(new, Debug, Default, Clone)]
pub struct EventPublisherImpl<Event: Debug> {
    logger: String,
    //todo переделать logger
    pub(crate) listener_map: HashMap<Discriminant<Event>, VecOfDomainEventListenerType<Event>>,
}

impl<Event: Debug + Clone + Hash + Eq> EventPublisherImpl<Event> {
    pub fn register_listener(&mut self, listener: impl DomainEventListener<Event> + 'static) {
        let event_type = listener.event_type();
        self.listener_map.entry(event_type).or_insert_with(|| {
            let vector: Vec<Arc<Mutex<(dyn DomainEventListener<Event> + 'static)>>> =
                vec![Arc::new(Mutex::new(listener))];
            vector
        });
    }

    fn send_events(
        &self,
        listeners: Vec<Arc<Mutex<dyn DomainEventListener<Event>>>>,
        event: Event,
    ) {
        for l in listeners {
            l.lock().unwrap().handle(&event);
        }
    }
}

impl<Event> DomainEventPublisher<Event> for EventPublisherImpl<Event>
where
    Event: Debug + Clone + 'static + Hash + Eq + Default,
{
    fn publish(&mut self, events: &Vec<Event>) {
        for e in events {
            let _ = &self
                .logger
                .push_str(format!("Processing event: {:?} \r\n", &e).as_mut_str());
            let listener_map = &self.listener_map;
            let e_type = discriminant(e);
            if listener_map.contains_key(&e_type) {
                let listeners_from_listener_map = listener_map.get(&e_type).unwrap();
                self.send_events(listeners_from_listener_map.to_vec(), e.clone())
            }
        }
    }
}
