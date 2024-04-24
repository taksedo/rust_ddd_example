use std::sync::{Arc, Mutex};

use actix_web::web::Data;
use common::types::main::base::generic_types::AM;
use lazy_static::lazy_static;
use usecase::main::{
    menu::{
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
    pub static ref ADD_MEAL_TO_MENU_USE_CASE: Data<AM<AddMealToMenuUseCase>> =
        Data::new(add_meal_to_menu_use_case()).clone();
    pub static ref GET_MEAL_BY_ID_USE_CASE: Data<AM<GetMealByIdUseCase>> =
        Data::new(get_meal_by_id_use_case().clone());
    pub static ref GET_MENU_USE_CASE: Data<AM<GetMenuUseCase>> =
        Data::new(get_menu_use_case()).clone();
    pub static ref REMOVE_MEAL_FROM_MENU_USECASE: Data<AM<RemoveMealFromMenuUseCase>> =
        Data::new(remove_meal_from_menu_usecase()).clone();
    pub static ref CANCEL_ORDER_USECASE: Data<AM<CancelOrderUseCase<ORepository, ORepository>>> =
        Data::new(cancel_order_usecase().clone());
    pub static ref CONFIRM_ORDER_USECASE: Data<AM<ConfirmOrderUseCase<ORepository, ORepository>>> =
        Data::new(confirm_order_usecase().clone());
    pub static ref GET_ORDER_BY_ID: Data<AM<GetOrderByIdUseCase<ORepository>>> =
        Data::new(get_order_by_id_usecase().clone());
    pub static ref GET_ORDERS_USECASE: Data<AM<GetOrdersUseCase<ORepository>>> =
        Data::new(get_orders_usecase().clone());
}

pub fn add_meal_to_menu_use_case() -> AM<AddMealToMenuUseCase> {
    let rule = MealAlreadyExistsUsesMealExtractor::new(MEAL_REPOSITORY.clone());

    let usecase = AddMealToMenuUseCase::new(
        MEAL_REPOSITORY.clone(),
        MEAL_ID_GENERATOR.clone(),
        Arc::new(Mutex::new(rule)),
    );
    Arc::new(Mutex::new(usecase))
}

pub fn get_meal_by_id_use_case() -> AM<GetMealByIdUseCase> {
    let usecase = GetMealByIdUseCase::new(MEAL_REPOSITORY.clone());
    Arc::new(Mutex::new(usecase))
}

pub fn get_menu_use_case() -> AM<GetMenuUseCase> {
    let usecase = GetMenuUseCase::new(MEAL_REPOSITORY.clone());
    Arc::new(Mutex::new(usecase))
}

pub fn remove_meal_from_menu_usecase() -> AM<RemoveMealFromMenuUseCase> {
    let usecase = RemoveMealFromMenuUseCase::new(MEAL_REPOSITORY.clone(), MEAL_REPOSITORY.clone());
    Arc::new(Mutex::new(usecase))
}

pub fn cancel_order_usecase() -> AM<CancelOrderUseCase<ORepository, ORepository>> {
    let usecase = CancelOrderUseCase::new(ORDER_REPOSITORY.clone(), ORDER_REPOSITORY.clone());
    Arc::new(Mutex::new(usecase))
}
pub fn confirm_order_usecase() -> AM<ConfirmOrderUseCase<ORepository, ORepository>> {
    let usecase = ConfirmOrderUseCase::new(ORDER_REPOSITORY.clone(), ORDER_REPOSITORY.clone());
    Arc::new(Mutex::new(usecase))
}

pub fn get_order_by_id_usecase() -> AM<GetOrderByIdUseCase<ORepository>> {
    let usecase = GetOrderByIdUseCase::new(ORDER_REPOSITORY.clone());
    Arc::new(Mutex::new(usecase))
}

pub fn get_orders_usecase() -> AM<GetOrdersUseCase<ORepository>> {
    let usecase = GetOrdersUseCase::new(ORDER_REPOSITORY.clone(), || GET_ORDERS_MAX_SIZE + 1);
    Arc::new(Mutex::new(usecase))
}
