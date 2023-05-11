use crate::main::menu::access::meal_extractor::MealExtractor;
use common_types::main::base::domain_event::DomainEventTrait;
use derive_new::new;
use domain::main::menu::meal_already_exists::MealAlreadyExists;
use domain::main::menu::meal_name::MealName;
use std::marker::PhantomData;

#[derive(new, Debug)]
pub struct MealAlreadyExistsImpl<E: DomainEventTrait + Clone, ME: MealExtractor<E>> {
    extractor: ME,
    phantom: PhantomData<E>,
}

impl<E, ME> MealAlreadyExists for MealAlreadyExistsImpl<E, ME>
where
    E: DomainEventTrait + Clone,
    ME: MealExtractor<E>,
{
    fn check(&mut self, name: MealName) -> bool {
        let meal = &self.extractor.get_by_name(name);
        meal.is_some() && !meal.clone().unwrap().removed
    }
}
