use std::fmt::Debug;

use domain::menu::{
    meal::Meal,
    value_objects::{meal_id::MealId, meal_name::MealName},
};

pub trait MealExtractor: Debug + Send {
    fn get_by_id(&mut self, id: &MealId) -> Option<Meal>;

    fn get_by_name(&mut self, name: &MealName) -> Option<Meal>;

    fn get_all(&mut self) -> Vec<Meal>;
}

impl dyn MealExtractor + 'static {
    pub fn downcast_ref<T: MealExtractor + 'static>(&self) -> Option<&T> {
        unsafe { Some(&*(self as *const dyn MealExtractor as *const T)) }
    }
}
