use smart_default::SmartDefault;

use domain::main::menu::value_objects::meal_description::MealDescription;
use domain::main::menu::value_objects::meal_id::MealId;
use domain::main::menu::value_objects::meal_name::MealName;
use domain::main::menu::value_objects::price::Price;
use domain::main::order::value_objects::shop_order_id::ShopOrderId;
use domain::test_fixtures::{
    rnd_meal, rnd_meal_description, rnd_meal_id, rnd_meal_name, rnd_price,
};
use usecase::main::menu::add_meal_to_menu::{AddMealToMenu, AddMealToMenuUseCaseError};
use usecase::main::menu::dto::meal_info::MealInfo;
use usecase::main::menu::get_meal_by_id::{GetMealById, GetMealByIdUseCaseError};
use usecase::main::menu::get_menu::GetMenu;
use usecase::main::menu::remove_meal_from_menu::{
    RemoveMealFromMenu, RemoveMealFromMenuUseCaseError,
};
use usecase::main::order::cancel_order::{CancelOrder, CancelOrderUseCaseError};
use usecase::main::order::confirm_order::{ConfirmOrder, ConfirmOrderUseCaseError};

const API_V1_TYPE_BASE_URL: &str = "http://localhost";

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
    pub fn verify_invoked(&self, name: MealName, description: MealDescription, price: Price) {
        assert_eq!(name, self.name.clone());
        assert_eq!(description, self.description.clone());
        assert_eq!(price, self.price.clone());
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

impl ConfirmOrder for MockConfirmOrder {
    fn execute(&mut self, order_id: ShopOrderId) -> Result<(), ConfirmOrderUseCaseError> {
        self.id = order_id;
        self.response
    }
}
