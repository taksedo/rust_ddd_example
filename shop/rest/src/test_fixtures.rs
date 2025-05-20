use async_trait::async_trait;
use domain::{
    menu::value_objects::{
        meal_description::MealDescription, meal_id::MealId, meal_name::MealName, price::Price,
    },
    order::{shop_order::OrderState, value_objects::shop_order_id::ShopOrderId},
    test_fixtures::*,
};
use smart_default::SmartDefault;
use usecase::{
    menu::{
        AddMealToMenu, AddMealToMenuUseCaseError, GetMealById, GetMealByIdUseCaseError, GetMenu,
        RemoveMealFromMenu, RemoveMealFromMenuUseCaseError, dto::meal_info::MealInfo,
    },
    order::{
        CancelOrder, CancelOrderUseCaseError, ConfirmOrder, ConfirmOrderUseCaseError, GetOrderById,
        GetOrderByIdUseCaseError, GetOrders, GetOrdersUseCaseError,
        dto::order_details::{AsDetails, OrderDetails},
    },
};

const API_V1_TYPE_BASE_URL: &str = "http://localhost";

pub fn rnd_order_details(order_state: OrderState) -> OrderDetails {
    order_with_state(order_state).as_details()
}

#[derive(SmartDefault, Debug)]
pub struct MockGetMenu {
    #[default(MealInfo::default())]
    pub meal_info: MealInfo,
}

#[async_trait]
impl GetMenu for MockGetMenu {
    async fn execute(&self) -> Vec<MealInfo> {
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

#[async_trait]
impl AddMealToMenu for MockAddMealToMenu {
    async fn execute(
        &mut self,
        name: &MealName,
        description: &MealDescription,
        price: &Price,
    ) -> Result<MealId, AddMealToMenuUseCaseError> {
        self.name = name.clone();
        self.description = description.clone();
        self.price = price.clone();
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

#[async_trait]
impl GetMealById for MockGetMealById {
    async fn execute(&mut self, id: &MealId) -> Result<MealInfo, GetMealByIdUseCaseError> {
        self.id = *id;
        self.response.clone()
    }
}

impl MockGetMealById {
    pub fn verify_invoked(&self, id: &MealId) {
        assert_eq!(self.id, *id)
    }
}

pub fn rnd_meal_info() -> MealInfo {
    let meal = rnd_meal();
    MealInfo {
        id: *meal.id(),
        name: meal.name().clone(),
        description: meal.description().clone(),
        price: meal.price().clone(),
        version: *meal.version(),
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
        self.with_parameter("id", &id.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, SmartDefault)]
pub struct MockRemoveMealFromMenu {
    #[default(Ok(()))]
    pub response: Result<(), RemoveMealFromMenuUseCaseError>,
    pub id: MealId,
}

#[async_trait]
impl RemoveMealFromMenu for MockRemoveMealFromMenu {
    async fn execute(&mut self, id: &MealId) -> Result<(), RemoveMealFromMenuUseCaseError> {
        self.id = *id;
        self.response
    }
}

#[derive(Debug, Clone, PartialEq, SmartDefault)]
pub struct MockCancelOrder {
    #[default(Ok(()))]
    pub response: Result<(), CancelOrderUseCaseError>,
    pub id: ShopOrderId,
}

#[async_trait]
impl CancelOrder for MockCancelOrder {
    async fn execute(&mut self, order_id: &ShopOrderId) -> Result<(), CancelOrderUseCaseError> {
        self.id = *order_id;
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

#[async_trait]
impl ConfirmOrder for MockConfirmOrder {
    async fn execute(&mut self, order_id: &ShopOrderId) -> Result<(), ConfirmOrderUseCaseError> {
        self.id = *order_id;
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

#[async_trait]
impl GetOrderById for MockGetOrderById {
    async fn execute(
        &mut self,
        id: &ShopOrderId,
    ) -> Result<OrderDetails, GetOrderByIdUseCaseError> {
        self.id = *id;
        self.clone().response
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MockGetOrders {
    pub response: Result<Vec<OrderDetails>, GetOrdersUseCaseError>,
    pub start_id: ShopOrderId,
    pub limit: usize,
}

#[async_trait]
impl GetOrders for MockGetOrders {
    async fn execute(
        &mut self,
        start_id: &ShopOrderId,
        limit: usize,
    ) -> Result<Vec<OrderDetails>, GetOrdersUseCaseError> {
        self.start_id = *start_id;
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
