use std::fmt::Debug;

use async_trait::async_trait;
use domain::menu::meal::Meal;

#[async_trait]
pub trait MealPersister: Debug + Send {
    async fn save(&mut self, meal: Meal);
}

impl dyn MealPersister + 'static {
    pub fn downcast_ref<T: MealPersister + 'static>(&self) -> Option<&T> {
        unsafe { Some(&*(self as *const dyn MealPersister as *const T)) }
    }
}
