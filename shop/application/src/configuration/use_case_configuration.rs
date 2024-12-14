use std::sync::LazyLock;

use actix_web::web::Data;
use common::types::base::{AM, AMW};
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

pub(super) static ADD_MEAL_TO_MENU_USE_CASE: LazyLock<Data<AM<AddMealToMenuUseCase>>> =
    LazyLock::new(|| Data::new(add_meal_to_menu_use_case()).clone());
pub(super) static GET_MEAL_BY_ID_USE_CASE: LazyLock<Data<AM<GetMealByIdUseCase>>> =
    LazyLock::new(|| Data::new(get_meal_by_id_use_case().clone()));
pub(super) static GET_MENU_USE_CASE: LazyLock<Data<AM<GetMenuUseCase>>> =
    LazyLock::new(|| Data::new(get_menu_use_case()).clone());
pub(super) static REMOVE_MEAL_FROM_MENU_USECASE: LazyLock<Data<AM<RemoveMealFromMenuUseCase>>> =
    LazyLock::new(|| Data::new(remove_meal_from_menu_usecase()).clone());
pub(super) static CANCEL_ORDER_USECASE: LazyLock<
    Data<AM<CancelOrderUseCase<ORepository, ORepository>>>,
> = LazyLock::new(|| Data::new(cancel_order_usecase().clone()));
pub(super) static CONFIRM_ORDER_USECASE: LazyLock<
    Data<AM<ConfirmOrderUseCase<ORepository, ORepository>>>,
> = LazyLock::new(|| Data::new(confirm_order_usecase().clone()));
pub(super) static GET_ORDER_BY_ID: LazyLock<Data<AM<GetOrderByIdUseCase<ORepository>>>> =
    LazyLock::new(|| Data::new(get_order_by_id_usecase().clone()));
pub(super) static GET_ORDERS_USECASE: LazyLock<Data<AM<GetOrdersUseCase<ORepository>>>> =
    LazyLock::new(|| Data::new(get_orders_usecase().clone()));

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
