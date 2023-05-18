use common_events::main::domain_event_publisher::DomainEventPublisher;
use common_types::main::base::domain_entity::DomainEntityTrait;
use derivative::Derivative;
use derive_new::new;
use domain::main::menu::meal::Meal;
use domain::main::menu::meal_id::MealId;
use domain::main::menu::meal_name::MealName;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
use usecase::main::menu::access::meal_extractor::MealExtractor;
use usecase::main::menu::access::meal_persister::MealPersister;

#[derive(new, Clone, Derivative, Debug)]
pub struct InMemoryMealRepository {
    pub event_publisher: Rc<RefCell<dyn DomainEventPublisher>>,
    #[new(value = "HashMap::new()")]
    pub storage: HashMap<MealId, Meal>,
}

impl MealPersister for InMemoryMealRepository {
    fn save(&mut self, meal: Meal) {
        self.event_publisher.borrow_mut().publish(meal.pop_events());
        self.storage.insert(meal.domain_entity_field.id, meal);
    }
}

// impl DomainEventPublisher for InMemoryMealRepository<P> {
//     fn publish<T: DomainEventTrait>(&mut self, events: Vec<T>) {
//         todo!()
//     }
// }

impl MealExtractor for InMemoryMealRepository {
    fn get_by_id(&mut self, id: MealId) -> Option<&Meal> {
        self.storage.get(&id)
    }

    fn get_by_name(&mut self, name: MealName) -> Option<Meal> {
        let storage: &HashMap<MealId, Meal> = &self.storage;
        storage.iter().find_map(|(_k, v)| {
            if v.name == name {
                Some(v.to_owned())
            } else {
                None
            }
        })
    }

    fn get_all(&mut self) -> Vec<Meal> {
        let storage: &HashMap<MealId, Meal> = &self.storage;
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
