use common_events::main::domain_event_publisher::DomainEventPublisher;
use common_types::main::base::domain_entity::DomainEntityTrait;
use common_types::main::base::domain_event::DomainEventTrait;
use derive_new::new;
use domain::main::menu::meal::Meal;
use domain::main::menu::meal_id::MealId;
use domain::main::menu::meal_name::MealName;
use std::collections::HashMap;
use std::fmt::Debug;
use usecase::main::menu::access::meal_extractor::MealExtractor;
use usecase::main::menu::access::meal_persister::MealPersister;

#[derive(new, Debug, Clone)]
pub struct InMemoryMealRepository<P, E>
where
    P: DomainEventPublisher<E>,
    E: DomainEventTrait + Clone,
{
    pub event_publisher: P,
    #[new(value = "HashMap::new()")]
    pub storage: HashMap<MealId, Meal<E>>,
}

impl<P, E> MealPersister<E> for InMemoryMealRepository<P, E>
where
    P: DomainEventPublisher<E>,
    E: DomainEventTrait + Clone,
{
    fn save(&mut self, meal: Meal<E>) {
        self.event_publisher.publish(meal.pop_events());
        self.storage.insert(meal.id, meal);
    }
}

// impl DomainEventPublisher for InMemoryMealRepository<P> {
//     fn publish<T: DomainEventTrait>(&mut self, events: Vec<T>) {
//         todo!()
//     }
// }

impl<P: DomainEventPublisher<E>, E: DomainEventTrait + Clone> MealExtractor<E>
    for InMemoryMealRepository<P, E>
{
    fn get_by_id(&mut self, id: MealId) -> Option<&Meal<E>> {
        self.storage.get(&id)
    }

    fn get_by_name(&mut self, name: MealName) -> Option<Meal<E>> {
        let storage: &HashMap<MealId, Meal<E>> = &self.storage;
        storage.iter().find_map(|(_k, v)| {
            if v.name == name {
                Some(v.to_owned())
            } else {
                None
            }
        })
    }

    fn get_all(&mut self) -> Vec<Meal<E>> {
        let storage: &HashMap<MealId, Meal<E>> = &self.storage;
        let mut all = vec![];
        for (&_k, v) in storage {
            if !v.to_owned().removed {
                all.push(v.to_owned())
            }
        }
        all
    }
}

// impl<D: DomainEventPublisher> MealPersister for InMemoryMealRepository<D> {
//     fn save(&mut self, mut meal: Meal) {
//         let popped_meal = meal.pop_events();
//         self.event_publisher.publish(popped_meal);
//         self.storage.insert(meal.id, meal);
//     }
// }
