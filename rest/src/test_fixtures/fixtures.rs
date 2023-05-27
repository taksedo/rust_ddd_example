use domain::main::menu::meal_description::MealDescription;
use domain::main::menu::meal_id::MealId;
use domain::main::menu::meal_name::MealName;
use domain::main::menu::price::Price;
use domain::test_fixtures::fixtures::{
    rnd_meal, rnd_meal_description, rnd_meal_id, rnd_meal_name, rnd_price,
};
use smart_default::SmartDefault;
use std::string::ToString;
use usecase::main::menu::add_meal_to_menu::{AddMealToMenu, AddMealToMenuUseCaseError};
use usecase::main::menu::dto::meal_info::MealInfo;
use usecase::main::menu::get_meal_by_id::{GetMealById, GetMealByIdUseCaseError};
use usecase::main::menu::get_menu::GetMenu;

const API_V1_TYPE_BASE_URL: &str = "http://localhost";

struct MockGetMenu {
    meal_info: MealInfo,
}

impl GetMenu for MockGetMenu {
    fn execute(&self) -> Vec<MealInfo> {
        vec![self.meal_info.clone()]
    }
}

#[derive(Debug)]
pub struct MockAddMealToMenu {
    pub(crate) response: Result<MealId, AddMealToMenuUseCaseError>,
    name: MealName, // lateinit var description: MealDescription
    // lateinit var price: Price
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
    pub fn verify_invoked(
        &self,
        name: MealName,
        // description: MealDescription,
        // price: Price,
    ) {
        assert_eq!(name, self.name.clone());
        // description shouldBe this.description
        // price shouldBe this.price
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
        id: meal.domain_entity_field.id.clone(),
        name: meal.name.clone(),
        version: meal.domain_entity_field.version.clone(),
    }
}

pub trait StringMethodsForRestTestExt {
    fn with_parameter(&mut self, name: String, value: String) -> String;

    fn with_host(&self) -> String;

    fn with_id(&mut self, id: u64) -> String;
}

impl StringMethodsForRestTestExt for String {
    fn with_parameter(&mut self, name: String, value: String) -> String {
        self.replace(&*format!("{{{name}}}"), &*format!("{value}"))
    }

    fn with_host(&self) -> String {
        format!("{API_V1_TYPE_BASE_URL}{self}")
    }

    fn with_id(&mut self, id: u64) -> String {
        self.with_parameter("id".to_string(), id.to_string())
    }
}
