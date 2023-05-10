use crate::main::menu::access::meal_persister::MealPersister;
use crate::main::menu::add_meal_to_menu::{
    AddMealToMenu, AddMealToMenuRequest, AddMealToMenuUseCaseError,
};
use common_types::main::base::domain_event::DomainEventTrait;
use derive_new::new;
use domain;
use domain::main::menu::meal::Meal;
use domain::main::menu::meal_id::{MealId, MealIdGenerator};
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(new, Debug)]
pub struct AddMealToMenuUseCase<'a, MP, I, E>
where
    MP: MealPersister<E>,
    I: MealIdGenerator,
    E: DomainEventTrait,
{
    pub meal_persister: &'a mut MP,
    pub id_generator: &'a I,
    // meal_exists: Rc<dyn MealAlreadyExists>,
    phantom: PhantomData<E>,
}

impl<'a, MP: MealPersister<E>, I: MealIdGenerator, E: DomainEventTrait> AddMealToMenu
    for AddMealToMenuUseCase<'a, MP, I, E>
{
    fn execute(
        &mut self,
        request: AddMealToMenuRequest,
    ) -> Result<MealId, AddMealToMenuUseCaseError> {
        Meal::add_meal_to_menu(self.id_generator, request.name)
            .map_err(|_| AddMealToMenuUseCaseError::AlreadyExists)
            .map(|new_meal_in_menu| {
                self.meal_persister.save(new_meal_in_menu);
                *self.id_generator.get_id()
            })
    }
}

impl<'a, MP: MealPersister<E>, I: MealIdGenerator, E: DomainEventTrait> DomainEventTrait
    for AddMealToMenuUseCase<'a, MP, I, E>
{
}
