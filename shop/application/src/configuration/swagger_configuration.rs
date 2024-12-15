use common::common_rest::{GenericErrorResponse, ValidationError};
use rest::{
    menu::{add_meal_to_menu_endpoint::AddMealToMenuRestRequest, meal_model::MealModel},
    order::order_model::{AddressModel, OrderItemModel, OrderModel},
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(title = "Rust DDD Example", description = "API Documentation"),
    paths(
        rest::menu::get_health_status::get_health_status,
        rest::menu::add_meal_to_menu_endpoint::add_meal_to_menu_endpoint,
        rest::menu::get_meal_by_id_endpoint::get_meal_by_id_endpoint,
        rest::menu::get_menu_endpoint::get_menu_endpoint,
        rest::menu::remove_meal_from_menu_endpoint::remove_meal_from_menu_endpoint,
        rest::order::get_orders_endpoint::get_orders_endpoint,
        rest::order::get_order_by_id_endpoint::get_order_by_id_endpoint,
        rest::order::cancel_order_endpoint::cancel_order_endpoint,
        rest::order::confirm_order_endpoint::confirm_order_endpoint,
    ),
    components(
        schemas(
            AddMealToMenuRestRequest,
            MealModel,
            GenericErrorResponse,
            ValidationError,
            OrderModel,
            OrderItemModel,
            AddressModel
        ),
        responses(MealModel, GenericErrorResponse, OrderModel)
    ),
    tags(
                (name = "Health", description = "Health check"),
                (name = "Meal", description = "All about Meal"),
                (name = "Order", description = "Operations with Order")
    )
)]
pub(crate) struct ApiDoc;
