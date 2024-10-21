use common::types::base::domain_entity::{DomainEntity, Version};

use super::{meal::Meal, value_objects::meal_name::MealName};
use crate::menu::{
    meal_events::MealEventEnum,
    value_objects::{meal_description::MealDescription, meal_id::MealId, price::Price},
};

pub struct MealRestorer {}

impl MealRestorer {
    pub fn restore_meal(
        id: &MealId,
        name: &MealName,
        description: &MealDescription,
        price: &Price,
        removed: bool,
        version: &Version,
        events: Vec<MealEventEnum>,
    ) -> Meal {
        Meal::with_all_args(
            DomainEntity::with_events(*id, *version, events),
            name.clone(),
            description.clone(),
            price.clone(),
            removed,
        )
    }
}

#[cfg(all(test, feature = "domain"))]
mod tests {
    use super::*;
    use crate::test_fixtures::{
        rnd_meal_description, rnd_meal_id, rnd_meal_name, rnd_price, version,
    };

    #[test]
    #[allow(non_snake_case)]
    fn restore_meal__success() {
        let mealId = &rnd_meal_id();
        let name = &rnd_meal_name();
        let description = &rnd_meal_description();
        let price = &rnd_price();
        let removed = &true;
        let version = &version();

        let mut meal: Meal =
            MealRestorer::restore_meal(mealId, name, description, price, *removed, version, vec![]);

        assert_eq!(meal.id(), mealId);
        assert_eq!(meal.name(), name);
        assert_eq!(meal.removed(), removed);
        assert_eq!(meal.version(), version);
        assert_eq!(meal.pop_events().len(), 0)
    }
}
