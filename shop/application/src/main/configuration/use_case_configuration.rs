use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use actix_web::web::Data;
use common::types::main::base::generic_types::AM;
use domain::main::menu::value_objects::meal_id::MealIdGenerator;
use lazy_static::lazy_static;
use usecase::main::{
    menu::{
        access::{meal_extractor::MealExtractor, meal_persister::MealPersister},
        invariant::meal_already_exists_uses_meal_extractor::MealAlreadyExistsUsesMealExtractor,
        scenario::{
            add_meal_to_menu_use_case::AddMealToMenuUseCase,
            get_meal_by_id_use_case::GetMealByIdUseCase, get_menu_use_case::GetMenuUseCase,
            remove_meal_from_menu_use_case::RemoveMealFromMenuUseCase,
        },
    },
    order::scenarios::{
        cancel_order_use_case::CancelOrderUseCase, confirm_order_use_case::ConfirmOrderUseCase,
        get_order_by_id_use_case::GetOrderByIdUseCase, get_orders_use_case::GetOrdersUseCase,
    },
};

use crate::main::configuration::persistence_configuration::{
    ORepository, MEAL_ID_GENERATOR, MEAL_REPOSITORY, ORDER_REPOSITORY,
};

const GET_ORDERS_MAX_SIZE: usize = 10;

lazy_static! {
    pub static ref ADD_MEAL_TO_MENU_USE_CASE: Data<AM<AddMealToMenuUseCase>> = Data::new(
        add_meal_to_menu_use_case(MEAL_REPOSITORY.clone() as _, MEAL_ID_GENERATOR.clone() as _,)
    )
    .clone();
    pub static ref GET_MEAL_BY_ID_USE_CASE: Data<AM<GetMealByIdUseCase>> =
        Data::new(get_meal_by_id_use_case(MEAL_REPOSITORY.clone()).clone());
    pub static ref GET_MENU_USE_CASE: Data<AM<GetMenuUseCase>> =
        Data::new(get_menu_use_case(MEAL_REPOSITORY.clone())).clone();
    pub static ref REMOVE_MEAL_FROM_MENU_USECASE: Data<AM<RemoveMealFromMenuUseCase>> =
        Data::new(remove_meal_from_menu_usecase(MEAL_REPOSITORY.clone())).clone();
    pub static ref CANCEL_ORDER_USECASE: Data<AM<CancelOrderUseCase<ORepository, ORepository>>> =
        Data::new(cancel_order_usecase(ORDER_REPOSITORY.clone()).clone());
    pub static ref CONFIRM_ORDER_USECASE: Data<AM<ConfirmOrderUseCase<ORepository, ORepository>>> =
        Data::new(confirm_order_usecase(ORDER_REPOSITORY.clone()).clone());
    pub static ref GET_ORDER_BY_ID: Data<AM<GetOrderByIdUseCase<ORepository>>> =
        Data::new(get_order_by_id_usecase(ORDER_REPOSITORY.clone()).clone());
    pub static ref GET_ORDERS_USECASE: Data<AM<GetOrdersUseCase<ORepository>>> =
        Data::new(get_orders_usecase(ORDER_REPOSITORY.clone()).clone());
}

pub fn add_meal_to_menu_use_case<U, V>(
    meal_repository: AM<U>,
    meal_id_generator: AM<V>,
) -> AM<AddMealToMenuUseCase>
where
    U: Debug + Send + MealExtractor + MealPersister + 'static,
    V: Debug + Send + MealIdGenerator + 'static,
{
    let rule = MealAlreadyExistsUsesMealExtractor::new(meal_repository.clone() as _);

    let usecase = AddMealToMenuUseCase::new(
        meal_repository.clone() as _,
        meal_id_generator.clone() as _,
        Arc::new(Mutex::new(rule)),
    );
    Arc::new(Mutex::new(usecase))
}

pub fn get_meal_by_id_use_case<U>(meal_repository: AM<U>) -> AM<GetMealByIdUseCase>
where
    U: Debug + Send + MealExtractor + MealPersister + 'static,
{
    let usecase = GetMealByIdUseCase::new(meal_repository.clone() as _);
    Arc::new(Mutex::new(usecase))
}

pub fn get_menu_use_case<U>(meal_repository: AM<U>) -> AM<GetMenuUseCase>
where
    U: Debug + Send + MealExtractor + MealPersister + 'static,
{
    let usecase = GetMenuUseCase::new(meal_repository.clone() as _);
    Arc::new(Mutex::new(usecase))
}

pub fn remove_meal_from_menu_usecase<U>(meal_repository: AM<U>) -> AM<RemoveMealFromMenuUseCase>
where
    U: Debug + Send + MealExtractor + MealPersister + 'static,
{
    let usecase =
        RemoveMealFromMenuUseCase::new(meal_repository.clone() as _, meal_repository.clone() as _);
    Arc::new(Mutex::new(usecase))
}

pub fn cancel_order_usecase(
    order_repository: AM<ORepository>,
) -> AM<CancelOrderUseCase<ORepository, ORepository>> {
    let usecase = CancelOrderUseCase::new(order_repository.clone(), order_repository.clone());
    Arc::new(Mutex::new(usecase))
}
pub fn confirm_order_usecase(
    order_repository: AM<ORepository>,
) -> AM<ConfirmOrderUseCase<ORepository, ORepository>> {
    let usecase = ConfirmOrderUseCase::new(order_repository.clone(), order_repository.clone());
    Arc::new(Mutex::new(usecase))
}

pub fn get_order_by_id_usecase(
    order_repository: AM<ORepository>,
) -> AM<GetOrderByIdUseCase<ORepository>> {
    let usecase = GetOrderByIdUseCase::new(order_repository.clone());
    Arc::new(Mutex::new(usecase))
}

pub fn get_orders_usecase(order_repository: AM<ORepository>) -> AM<GetOrdersUseCase<ORepository>> {
    let usecase = GetOrdersUseCase::new(order_repository.clone(), || GET_ORDERS_MAX_SIZE + 1);
    Arc::new(Mutex::new(usecase))
}
