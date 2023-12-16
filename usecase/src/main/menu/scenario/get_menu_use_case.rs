use std::sync::{Arc, Mutex};

use derive_new::new;

use crate::main::menu::{
    access::meal_extractor::MealExtractor, dto::meal_info::MealInfo, get_menu::GetMenu,
};

#[derive(Debug, new)]
pub struct GetMenuUseCase {
    pub(crate) meal_extractor: Arc<Mutex<dyn MealExtractor>>,
}

impl GetMenu for GetMenuUseCase {
    fn execute(&self) -> Vec<MealInfo> {
        self.meal_extractor
            .lock()
            .unwrap()
            .get_all()
            .into_iter()
            .map(MealInfo::from)
            .collect()
    }
}
