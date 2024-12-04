use std::collections::HashMap;

use common::types::{
    base::domain_entity::{DomainEntity, Version},
    common::count::Count,
};
use time::OffsetDateTime;

use crate::{
    cart::{
        cart::Cart,
        value_objects::{cart_id::CartId, customer_id::CustomerId},
    },
    menu::value_objects::meal_id::MealId,
};

pub struct CartRestorer {}

impl CartRestorer {
    pub fn restore_cart(
        id: CartId,
        for_customer: CustomerId,
        created: OffsetDateTime,
        meals: HashMap<MealId, Count>,
        version: Version,
    ) -> Cart {
        Cart {
            entity_params: DomainEntity {
                id,
                version,
                events: vec![],
            },
            for_customer,
            created,
            meals,
        }
    }
}

#[cfg(test)]
mod tests {
    use common::test_fixtures::rnd_count;

    use super::*;
    use crate::test_fixtures::{rnd_cart_id, rnd_customer_id, rnd_meal_id, version};

    #[test]
    fn restore_cart_success() {
        let cart_id = rnd_cart_id();
        let guest_id = rnd_customer_id();
        let version = version();
        let meals = HashMap::from([(rnd_meal_id(), rnd_count())]);
        let created = OffsetDateTime::now_utc();
        let cart = CartRestorer::restore_cart(cart_id, guest_id, created, meals.clone(), version);

        assert_eq!(cart.id(), &cart_id);
        assert_eq!(cart.for_customer(), &guest_id);
        assert_eq!(cart.version(), &version);
        assert_eq!(cart.meals(), &meals);
    }
}
