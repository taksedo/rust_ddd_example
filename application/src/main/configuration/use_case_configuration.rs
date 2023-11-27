use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use actix_web::web::Data;
use lazy_static::lazy_static;

use domain::main::menu::value_objects::meal_id::MealIdGenerator;
use usecase::main::menu::access::meal_extractor::MealExtractor;
use usecase::main::menu::access::meal_persister::MealPersister;
use usecase::main::menu::invariant::meal_already_exists_uses_meal_extractor::MealAlreadyExistsUsesMealExtractor;
use usecase::main::menu::scenario::add_meal_to_menu_use_case::AddMealToMenuUseCase;
use usecase::main::menu::scenario::get_meal_by_id_use_case::GetMealByIdUseCase;
use usecase::main::menu::scenario::get_menu_use_case::GetMenuUseCase;
use usecase::main::menu::scenario::remove_meal_from_menu_use_case::RemoveMealFromMenuUseCase;

use crate::main::configuration::persistence_configuration::{MEAL_ID_GENERATOR, MEAL_REPOSITORY};

lazy_static! {
    pub static ref ADD_MEAL_TO_MENU_USE_CASE: Data<Arc<Mutex<AddMealToMenuUseCase>>> =
        Data::new(Arc::clone(&add_meal_to_menu_use_case(
            Arc::clone(&MEAL_REPOSITORY) as _,
            Arc::clone(&MEAL_ID_GENERATOR) as _,
        )));
    pub static ref GET_MEAL_BY_ID_USE_CASE: Data<Arc<Mutex<GetMealByIdUseCase>>> = Data::new(
        Arc::clone(&get_meal_by_id_use_case(Arc::clone(&MEAL_REPOSITORY,)))
    );
    pub static ref GET_MENU_USE_CASE: Data<Arc<Mutex<GetMenuUseCase>>> = Data::new(Arc::clone(
        &get_menu_use_case(Arc::clone(&MEAL_REPOSITORY,))
    ));
    pub static ref REMOVE_MEAL_FROM_MENU_USECASE: Data<Arc<Mutex<RemoveMealFromMenuUseCase>>> =
        Data::new(Arc::clone(&remove_meal_from_menu_usecase(Arc::clone(
            &MEAL_REPOSITORY,
        ))));
}

pub fn add_meal_to_menu_use_case<U, V>(
    meal_repository: Arc<Mutex<U>>,
    meal_id_generator: Arc<Mutex<V>>,
) -> Arc<Mutex<AddMealToMenuUseCase>>
where
    U: Debug + Send + MealExtractor + MealPersister + 'static,
    V: Debug + Send + MealIdGenerator + 'static,
{
    let rule = MealAlreadyExistsUsesMealExtractor::new(Arc::clone(&meal_repository) as _);

    let usecase = AddMealToMenuUseCase::new(
        Arc::clone(&meal_repository) as _,
        meal_id_generator,
        Arc::new(Mutex::new(rule)),
    );
    Arc::new(Mutex::new(usecase))
}

pub fn get_meal_by_id_use_case<U>(meal_repository: Arc<Mutex<U>>) -> Arc<Mutex<GetMealByIdUseCase>>
where
    U: Debug + Send + MealExtractor + MealPersister + 'static,
{
    let usecase = GetMealByIdUseCase::new(Arc::clone(&meal_repository) as _);
    Arc::new(Mutex::new(usecase))
}

pub fn get_menu_use_case<U>(meal_repository: Arc<Mutex<U>>) -> Arc<Mutex<GetMenuUseCase>>
where
    U: Debug + Send + MealExtractor + MealPersister + 'static,
{
    let usecase = GetMenuUseCase::new(Arc::clone(&meal_repository) as _);
    Arc::new(Mutex::new(usecase))
}

pub fn remove_meal_from_menu_usecase<U>(
    meal_repository: Arc<Mutex<U>>,
) -> Arc<Mutex<RemoveMealFromMenuUseCase>>
where
    U: Debug + Send + MealExtractor + MealPersister + 'static,
{
    let usecase = RemoveMealFromMenuUseCase::new(
        Arc::clone(&meal_repository) as _,
        Arc::clone(&meal_repository) as _,
    );
    Arc::new(Mutex::new(usecase))
}
