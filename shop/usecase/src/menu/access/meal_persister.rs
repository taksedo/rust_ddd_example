use std::fmt::Debug;

use domain::menu::meal::Meal;

pub trait MealPersister: Debug + Send {
    fn save(&mut self, meal: Meal);
}

impl dyn MealPersister + 'static {
    pub fn downcast_ref<T: MealPersister + 'static>(&self) -> Option<&T> {
        unsafe { Some(&*(self as *const dyn MealPersister as *const T)) }
    }
}
