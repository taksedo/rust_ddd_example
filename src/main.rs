#![allow(unused_imports)]

use common_types::main::base::domain_entity::Version;
use domain::main::menu::meal::Meal;
use domain::main::menu::meal_id::{MealId, MealIdGenerator};
use domain::main::menu::meal_name::MealName;
use in_memory_persistence::main::menu::in_memory_incremental_meal_id_generator::InMemoryIncrementalMealIdGenerator;
use std::rc::Rc;

use usecase::main::menu::add_meal_to_menu::{AddMealToMenu, AddMealToMenuRequest};
use usecase::main::menu::scenario::add_meal_to_menu_use_case::AddMealToMenuUseCase;

fn main() {
    let meal_id_generator = InMemoryIncrementalMealIdGenerator::new();
    let _meal_id = meal_id_generator.generate();
    let _meal_id = meal_id_generator.generate();
    let meal_id = meal_id_generator.generate();
    println!("meal_id => {:?}", meal_id);
    let _meal_name = String::from("My Meal Name");
    // let meal = Meal::new(meal_id, MealName::from(meal_name).unwrap(), Version::new());
    // println!("meal => {:?}", meal);
    // let _meal_request =
    AddMealToMenuRequest::new(MealName::from(String::from("Meal Name for request")).unwrap());
}
