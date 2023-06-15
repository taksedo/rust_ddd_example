use crate::main::menu::meal_events::MealAddedToMenuDomainEventType;
use crate::test_fixtures::fixtures::rnd_meal_id;
use common_types::main::base::domain_event::DomainEventNew;

#[test]
fn test_event() {
    let a: DomainEventNew<MealAddedToMenuDomainEventType> =
        dbg!(DomainEventNew::new().id(rnd_meal_id().to_i64()).build());
}
