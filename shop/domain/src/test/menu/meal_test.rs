#![allow(non_snake_case)]

use std::sync::{atomic::AtomicI64, Arc, Mutex};

use derive_new::new;

use crate::{
    main::menu::{
        meal::{Meal, MealError::AlreadyExistsWithSameNameError},
        meal_already_exists::MealAlreadyExists,
        meal_events::{MealAddedToMenuDomainEvent, MealEventEnum, MealRemovedFromMenuDomainEvent},
        value_objects::{
            meal_id::{MealId, MealIdGenerator},
            meal_name::MealName,
        },
    },
    test_fixtures::{
        print_type_of, rnd_meal, rnd_meal_description, rnd_meal_id, rnd_meal_name, rnd_price,
        rnd_removed_meal,
    },
};
