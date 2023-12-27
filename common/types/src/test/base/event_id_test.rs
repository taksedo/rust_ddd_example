use derive_new::new;

use crate::main::base::domain_event::DomainEvent;

#[allow(non_snake_case)]
#[test]
fn create_event__check_event_id_is_unique() {
    let firstEvent = EmptyEvent::new();
    let secondEvent = EmptyEvent::new();
    assert_ne!(
        firstEvent.domain_events_params.id,
        secondEvent.domain_events_params.id
    );
    assert_ne!(
        firstEvent.domain_events_params.id.value,
        secondEvent.domain_events_params.id.value
    )
}

#[derive(new)]
struct EmptyEvent {
    #[new(value = "DomainEvent::default()")]
    domain_events_params: DomainEvent,
}
