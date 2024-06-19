use std::collections::HashMap;

use common::types::test_fixtures::rnd_count;
use time::OffsetDateTime;

use crate::{
    main::cart::cart_restorer::CartRestorer,
    test_fixtures::{rnd_cart_id, rnd_customer_id, rnd_meal_id, version},
};

#[test]
fn restore_cart_success() {
    let cart_id = rnd_cart_id();
    let guest_id = rnd_customer_id();
    let version = version();
    let meals = HashMap::from([(rnd_meal_id(), rnd_count())]);
    let created = OffsetDateTime::now_utc();
    let cart = CartRestorer::restore_cart(cart_id, guest_id, created, meals.clone(), version);

    assert_eq!(cart.get_id(), &cart_id);
    assert_eq!(cart.get_for_customer(), &guest_id);
    assert_eq!(cart.get_version(), &version);
    assert_eq!(cart.get_meals(), &meals);
}
