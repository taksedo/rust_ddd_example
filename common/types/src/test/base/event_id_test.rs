use crate::main::base::domain_event::EventId;
use time::OffsetDateTime;

#[allow(non_snake_case)]
#[test]
fn create_event__check_event_id_is_unique() {
    let firstEvent = EmptyEvent::new();
    let secondEvent = EmptyEvent::new();
    assert_ne!(firstEvent.id, secondEvent.id);
    assert_ne!(firstEvent.id.value, secondEvent.id.value)
}

struct EmptyEvent {
    id: EventId,
    _created: OffsetDateTime,
}

impl EmptyEvent {
    pub fn new() -> Self {
        Self {
            id: EventId::new(),
            _created: OffsetDateTime::now_utc(),
        }
    }
}
