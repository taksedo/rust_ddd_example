use domain::{
    main::{
        menu::value_objects::{
            meal_description::MealDescription, meal_id::MealId, meal_name::MealName, price::Price,
        },
        order::{shop_order::OrderState, value_objects::shop_order_id::ShopOrderId},
    },
    test_fixtures::{
        order_with_state, rnd_meal, rnd_meal_description, rnd_meal_id, rnd_meal_name, rnd_price,
    },
};
use smart_default::SmartDefault;
use usecase::main::{
    menu::{
        add_meal_to_menu::{AddMealToMenu, AddMealToMenuUseCaseError},
        dto::meal_info::MealInfo,
        get_meal_by_id::{GetMealById, GetMealByIdUseCaseError},
        get_menu::GetMenu,
        remove_meal_from_menu::{RemoveMealFromMenu, RemoveMealFromMenuUseCaseError},
    },
    order::{
        cancel_order::{CancelOrder, CancelOrderUseCaseError},
        confirm_order::{ConfirmOrder, ConfirmOrderUseCaseError},
        dto::order_details::{OrderDetails, ToDetails},
        get_order_by_id::{GetOrderById, GetOrderByIdUseCaseError},
        get_orders::{GetOrders, GetOrdersUseCaseError},
    },
};

const API_V1_TYPE_BASE_URL: &str = "http://localhost";

pub fn rnd_order_details(order_state: OrderState) -> OrderDetails {
    order_with_state(order_state).to_details()
}

#[derive(SmartDefault, Debug)]
pub struct MockGetMenu {
    #[default(MealInfo::default())]
    pub meal_info: MealInfo,
}

impl GetMenu for MockGetMenu {
    fn execute(&self) -> Vec<MealInfo> {
        vec![self.meal_info.clone()]
    }
}

#[derive(Debug)]
pub struct MockAddMealToMenu {
    pub(crate) response: Result<MealId, AddMealToMenuUseCaseError>,
    name: MealName,
    pub description: MealDescription,
    pub price: Price,
}

impl Default for MockAddMealToMenu {
    fn default() -> Self {
        Self {
            response: Ok(rnd_meal_id()),
            name: rnd_meal_name(),
            description: rnd_meal_description(),
            price: rnd_price(),
        }
    }
}

impl AddMealToMenu for MockAddMealToMenu {
    fn execute(
        &mut self,
        name: MealName,
        description: MealDescription,
        price: Price,
    ) -> Result<MealId, AddMealToMenuUseCaseError> {
        self.name = name;
        self.description = description;
        self.price = price;
        self.response.to_owned()
    }
}

impl MockAddMealToMenu {
    pub fn verify_invoked(&self, name: &MealName, description: &MealDescription, price: &Price) {
        assert_eq!(name, &self.name);
        assert_eq!(description, &self.description);
        assert_eq!(price, &self.price);
    }
}

#[derive(Debug, Clone, PartialEq, SmartDefault)]
pub struct MockGetMealById {
    #[default(Ok(MealInfo::default()))]
    pub response: Result<MealInfo, GetMealByIdUseCaseError>,
    pub id: MealId,
}

impl GetMealById for MockGetMealById {
    fn execute(&mut self, id: MealId) -> Result<MealInfo, GetMealByIdUseCaseError> {
        self.id = id;
        self.response.clone()
    }
}

impl MockGetMealById {
    pub fn verify_invoked(&self, id: MealId) {
        assert_eq!(self.id, id)
    }
}

pub fn rnd_meal_info() -> MealInfo {
    let meal = rnd_meal();
    MealInfo {
        id: meal.entity_params.id,
        name: meal.name.clone(),
        description: meal.description.clone(),
        price: meal.price.clone(),
        version: meal.entity_params.version,
    }
}

pub trait StringMethodsForRestTestExt {
    fn with_host(&self) -> String;

    fn with_parameter(&self, name: &str, value: &str) -> String;

    fn with_id(&self, id: &i64) -> String;
}

impl StringMethodsForRestTestExt for String {
    fn with_host(&self) -> String {
        format!("{API_V1_TYPE_BASE_URL}{self}")
    }

    fn with_parameter(&self, name: &str, value: &str) -> String {
        self.replace(&*format!("{{{name}}}"), value)
    }

    fn with_id(&self, id: &i64) -> String {
        self.with_parameter("id", id.to_string().as_str())
    }
}

#[derive(Debug, Clone, PartialEq, SmartDefault)]
pub struct MockRemoveMealFromMenu {
    #[default(Ok(()))]
    pub response: Result<(), RemoveMealFromMenuUseCaseError>,
    pub id: MealId,
}

impl RemoveMealFromMenu for MockRemoveMealFromMenu {
    fn execute(&mut self, id: MealId) -> Result<(), RemoveMealFromMenuUseCaseError> {
        self.id = id;
        self.response
    }
}

#[derive(Debug, Clone, PartialEq, SmartDefault)]
pub struct MockCancelOrder {
    #[default(Ok(()))]
    pub response: Result<(), CancelOrderUseCaseError>,
    pub id: ShopOrderId,
}

impl CancelOrder for MockCancelOrder {
    fn execute(&mut self, order_id: ShopOrderId) -> Result<(), CancelOrderUseCaseError> {
        self.id = order_id;
        self.response
    }
}

#[derive(Debug, Clone, PartialEq, SmartDefault)]
pub struct MockConfirmOrder {
    #[default(Ok(()))]
    pub response: Result<(), ConfirmOrderUseCaseError>,
    pub id: ShopOrderId,
}

impl MockConfirmOrder {
    pub fn verify_invoked(&self, id: &ShopOrderId) {
        assert_eq!(&self.id, id);
    }
}

impl ConfirmOrder for MockConfirmOrder {
    fn execute(&mut self, order_id: ShopOrderId) -> Result<(), ConfirmOrderUseCaseError> {
        self.id = order_id;
        self.response
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MockGetOrderById {
    pub response: Result<OrderDetails, GetOrderByIdUseCaseError>,
    pub id: ShopOrderId,
}

impl MockGetOrderById {
    pub fn verify_invoked(&self, id: &ShopOrderId) {
        assert_eq!(&self.id, id);
    }
}

impl GetOrderById for MockGetOrderById {
    fn execute(&mut self, id: ShopOrderId) -> Result<OrderDetails, GetOrderByIdUseCaseError> {
        self.id = id;
        self.clone().response
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MockGetOrders {
    pub response: Result<Vec<OrderDetails>, GetOrdersUseCaseError>,
    pub start_id: ShopOrderId,
    pub limit: usize,
}

impl GetOrders for MockGetOrders {
    fn execute(
        &mut self,
        start_id: ShopOrderId,
        limit: usize,
    ) -> Result<Vec<OrderDetails>, GetOrdersUseCaseError> {
        self.start_id = start_id;
        self.limit = limit;
        self.response.clone()
    }
}

impl MockGetOrders {
    pub fn verify_invoked(&self, start_id: &ShopOrderId, limit: &usize) {
        assert_eq!(&self.start_id, start_id);
        assert_eq!(&self.limit, limit);
    }
}
