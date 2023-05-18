use common_types::main::base::domain_event::DomainEventTrait;

pub trait DomainEventListener<T: DomainEventTrait> {
    fn event_type() -> T;
    fn handle(event: T);
}
