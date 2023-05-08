use derive_new::new;
use domain::main::menu::meal::Meal;
use domain::main::menu::meal_id::MealId;
use std::collections::HashMap;

#[derive(new, Debug, Clone)]
pub struct InMemoryMealRepository {
    // pub event_publisher: DomainEventPublisher,
    #[new(value = "HashMap::new()")]
    pub storage: HashMap<MealId, Meal>,
}

// impl<D: DomainEventPublisher> InMemoryMealRepository<D> {}
//
// impl<D: DomainEventPublisher> MealExtractor for InMemoryMealRepository<D> {
//     fn get_by_id(&mut self, id: MealId) -> Option<Meal> {
//         self.storage.get(&id).map(|v| v.to_owned())
//     }
//
//     fn get_by_name(&mut self, name: MealName) -> Option<Meal> {
//         let storage: &HashMap<MealId, Meal> = &self.storage;
//         storage.iter().find_map(|(_k, v)| {
//             if v.name == name {
//                 Some(v.to_owned())
//             } else {
//                 None
//             }
//         })
//     }
//
//     fn get_all(&mut self) -> Vec<Meal> {
//         let storage: &HashMap<MealId, Meal> = &self.storage;
//         let mut all = vec![];
//         for (&_k, v) in storage {
//             if !v.to_owned().removed {
//                 all.push(v.to_owned())
//             }
//         }
//         all
//     }
// }

// impl<D: DomainEventPublisher> MealPersister for InMemoryMealRepository<D> {
//     fn save(&mut self, mut meal: Meal) {
//         let popped_meal = meal.pop_events();
//         self.event_publisher.publish(popped_meal);
//         self.storage.insert(meal.id, meal);
//     }
// }
