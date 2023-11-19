use crate::main::menu::access::meal_extractor::MealExtractor;
use crate::main::menu::dto::meal_info::MealInfo;
use crate::main::menu::get_menu::GetMenu;
use derive_new::new;
use std::sync::{Arc, Mutex};

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
