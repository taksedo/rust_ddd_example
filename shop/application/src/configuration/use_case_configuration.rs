use actix_web::web::Data;
use common::types::base::{AM, AMW};
use lazy_static::lazy_static;
use usecase::{
    menu::{
        invariant::meal_already_exists_uses_meal_extractor::MealAlreadyExistsUsesMealExtractor,
        scenario::{
            AddMealToMenuUseCase, GetMealByIdUseCase, GetMenuUseCase, RemoveMealFromMenuUseCase,
        },
    },
    order::scenarios::{
        CancelOrderUseCase, ConfirmOrderUseCase, GetOrderByIdUseCase, GetOrdersUseCase,
    },
};

use crate::configuration::persistence_configuration::{
    ORepository, MEAL_ID_GENERATOR, MEAL_REPOSITORY, ORDER_REPOSITORY,
};

const GET_ORDERS_MAX_SIZE: usize = 10;

lazy_static! {
    pub(super) static ref ADD_MEAL_TO_MENU_USE_CASE: Data<AM<AddMealToMenuUseCase>> =
        Data::new(add_meal_to_menu_use_case()).clone();
    pub(super) static ref GET_MEAL_BY_ID_USE_CASE: Data<AM<GetMealByIdUseCase>> =
        Data::new(get_meal_by_id_use_case().clone());
    pub(super) static ref GET_MENU_USE_CASE: Data<AM<GetMenuUseCase>> =
        Data::new(get_menu_use_case()).clone();
    pub(super) static ref REMOVE_MEAL_FROM_MENU_USECASE: Data<AM<RemoveMealFromMenuUseCase>> =
        Data::new(remove_meal_from_menu_usecase()).clone();
    pub(super) static ref CANCEL_ORDER_USECASE: Data<AM<CancelOrderUseCase<ORepository, ORepository>>> =
        Data::new(cancel_order_usecase().clone());
    pub(super) static ref CONFIRM_ORDER_USECASE: Data<AM<ConfirmOrderUseCase<ORepository, ORepository>>> =
        Data::new(confirm_order_usecase().clone());
    pub(super) static ref GET_ORDER_BY_ID: Data<AM<GetOrderByIdUseCase<ORepository>>> =
        Data::new(get_order_by_id_usecase().clone());
    pub(super) static ref GET_ORDERS_USECASE: Data<AM<GetOrdersUseCase<ORepository>>> =
        Data::new(get_orders_usecase().clone());
}

fn add_meal_to_menu_use_case() -> AM<AddMealToMenuUseCase> {
    let rule = MealAlreadyExistsUsesMealExtractor::new(MEAL_REPOSITORY.clone());

    let usecase = AddMealToMenuUseCase::new(
        MEAL_REPOSITORY.clone(),
        MEAL_ID_GENERATOR.clone(),
        AMW::new(rule),
    );
    AMW::new(usecase)
}

fn get_meal_by_id_use_case() -> AM<GetMealByIdUseCase> {
    let usecase = GetMealByIdUseCase::new(MEAL_REPOSITORY.clone());
    AMW::new(usecase)
}

fn get_menu_use_case() -> AM<GetMenuUseCase> {
    let usecase = GetMenuUseCase::new(MEAL_REPOSITORY.clone());
    AMW::new(usecase)
}

fn remove_meal_from_menu_usecase() -> AM<RemoveMealFromMenuUseCase> {
    let usecase = RemoveMealFromMenuUseCase::new(MEAL_REPOSITORY.clone(), MEAL_REPOSITORY.clone());
    AMW::new(usecase)
}

fn cancel_order_usecase() -> AM<CancelOrderUseCase<ORepository, ORepository>> {
    let usecase = CancelOrderUseCase::new(ORDER_REPOSITORY.clone(), ORDER_REPOSITORY.clone());
    AMW::new(usecase)
}
fn confirm_order_usecase() -> AM<ConfirmOrderUseCase<ORepository, ORepository>> {
    let usecase = ConfirmOrderUseCase::new(ORDER_REPOSITORY.clone(), ORDER_REPOSITORY.clone());
    AMW::new(usecase)
}

fn get_order_by_id_usecase() -> AM<GetOrderByIdUseCase<ORepository>> {
    let usecase = GetOrderByIdUseCase::new(ORDER_REPOSITORY.clone());
    AMW::new(usecase)
}

fn get_orders_usecase() -> AM<GetOrdersUseCase<ORepository>> {
    let usecase = GetOrdersUseCase::new(ORDER_REPOSITORY.clone(), || GET_ORDERS_MAX_SIZE + 1);
    AMW::new(usecase)
}
