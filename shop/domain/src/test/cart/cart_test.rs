use std::{
    collections::HashMap,
    mem::discriminant,
    sync::{Arc, Mutex},
};

use common::types::{common::count::Count, test_fixtures::rnd_count};
use smart_default::SmartDefault;
use time::OffsetDateTime;

use crate::{
    main::cart::{
        cart::Cart,
        cart_events::{CartCreatedDomainEvent, CartEventEnum, MealAddedToCartDomainEvent},
        value_objects::cart_id::{CartId, CartIdGenerator},
    },
    test_fixtures::{rnd_cart, rnd_cart_id, rnd_customer_id, rnd_meal},
};
