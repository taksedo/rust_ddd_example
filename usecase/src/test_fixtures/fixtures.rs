use crate::main::menu::access::meal_extractor::MealExtractor;
use crate::main::menu::access::meal_persister::MealPersister;
use common_types::main::base::domain_entity::DomainEntityTrait;
use derive_new::new;
use domain::main::menu::meal::Meal;
use domain::main::menu::meal_description::MealDescription;
use domain::main::menu::meal_events::DomainEventEnum;
use domain::main::menu::meal_events::MealRemovedFromMenuDomainEvent;
use domain::main::menu::meal_id::MealId;
use domain::main::menu::meal_name::MealName;
use domain::main::menu::price::Price;
use domain::test_fixtures::fixtures::rnd_meal;
use std::any::Any;

pub fn removed_meal() -> Meal {
    let mut meal = rnd_meal();
    meal.remove_meal_from_menu();
    meal
}

#[derive(new, Debug, Clone)]
pub struct MockMealPersister {
    #[new(value = "None")]
    pub meal: Option<Meal>,
}

impl MockMealPersister {
    pub fn verify_invoked(
        &self,
        id: Option<MealId>,
        name: Option<MealName>,
        description: Option<MealDescription>,
        price: Option<Price>,
    ) {
        if id.is_some() {
            assert_eq!(
                self.to_owned().meal.unwrap().domain_entity_field.id,
                id.unwrap()
            )
        }
        if name.is_some() {
            assert_eq!(self.to_owned().meal.unwrap().name, name.unwrap())
        }
        if description.is_some() {
            assert_eq!(
                self.to_owned().meal.unwrap().description,
                description.unwrap()
            )
        }
        if price.is_some() {
            assert_eq!(self.to_owned().meal.unwrap().price, price.unwrap())
        }
    }

    pub fn verify_invoked_meal(&self, meal: Option<Meal>) {
        if meal.is_some() {
            assert_eq!(self.to_owned().meal, meal)
        }
    }

    pub fn verify_events_after_deletion(&mut self, id: MealId) {
        let event_enum: DomainEventEnum = MealRemovedFromMenuDomainEvent::new(id).into();
        let events = self
            .to_owned()
            .meal
            .unwrap()
            .pop_events()
            .get(0)
            .unwrap()
            .clone();
        assert_eq!(events.type_id(), event_enum.type_id());
    }

    pub fn verify_empty(&self) {
        assert!(&self.meal.is_none());
    }
}

impl MealPersister for MockMealPersister {
    fn save(&mut self, meal: Meal) {
        self.meal = Some(meal);
    }
}

#[derive(new, Clone, PartialEq, Debug, Default)]
pub struct MockMealExtractor {
    #[new(default)]
    pub meal: Option<Meal>,
    #[new(default)]
    pub id: Option<MealId>,
    #[new(default)]
    pub name: Option<MealName>,
    #[new(default)]
    pub all: bool,
}

impl MealExtractor for MockMealExtractor {
    fn get_by_id(&mut self, id: MealId) -> Option<Meal> {
        self.id = Option::from(id);
        if Some(&self.meal).is_some() && self.id == Some(id) {
            self.clone().meal
        } else {
            None
        }
    }

    fn get_by_name(&mut self, name: MealName) -> Option<Meal> {
        self.name = Option::from(name.to_owned());
        if Some(&self.meal).is_some() && self.to_owned().name.unwrap() == name {
            self.to_owned().meal
        } else {
            None
        }
    }

    fn get_all(&mut self) -> Vec<Meal> {
        self.all = true;
        if self.meal.is_some() {
            vec![self.to_owned().meal.unwrap()]
        } else {
            vec![]
        }
    }
}

impl MockMealExtractor {
    pub fn verify_invoked_get_by_id(&self, id: MealId) {
        assert_eq!(&self.id.unwrap(), &id);
        assert!(!&self.all);
        assert!(&self.name.is_none());
    }

    pub fn verify_invoked_get_by_name(&self, name: MealName) {
        assert_eq!(&self.to_owned().name.unwrap(), &name);
        assert!(!&self.all);
        assert!(&self.id.is_none());
    }

    pub fn verify_invoked_get_all(&self) {
        assert!(&self.all);
        assert!(&self.id.is_none());
        assert!(&self.name.is_none());
    }

    pub fn verify_empty(&self) {
        assert!(&self.name.is_none());
    }
}

impl dyn MealExtractor + 'static {
    pub fn downcast_ref<T: MealExtractor + 'static>(&self) -> Option<&T> {
        unsafe { Some(&*(self as *const dyn MealExtractor as *const T)) }
    }
}

impl dyn MealPersister + 'static {
    pub fn downcast_ref<T: MealPersister + 'static>(&self) -> Option<&T> {
        unsafe { Some(&*(self as *const dyn MealPersister as *const T)) }
    }
}
