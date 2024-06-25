use crate::menu::dto::meal_info::MealInfo;

pub trait GetMenu {
    fn execute(&self) -> Vec<MealInfo>;
}
