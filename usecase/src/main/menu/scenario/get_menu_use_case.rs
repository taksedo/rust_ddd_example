use crate::main::menu::access::meal_extractor::MealExtractor;
use crate::main::menu::dto::meal_info::MealInfo;
use crate::main::menu::get_menu::GetMenu;
use derive_new::new;
use domain::main::menu::meal::Meal;
use std::sync::{Arc, Mutex};

#[derive(Debug, new)]
pub struct GetMenuUseCase {
    meal_extractor: Arc<Mutex<dyn MealExtractor>>,
}

impl GetMenu for GetMenuUseCase {
    fn execute(&self) -> Vec<MealInfo> {
        self.meal_extractor
            .lock()
            .unwrap()
            .get_all()
            .into_iter()
            .map(|s: Meal| MealInfo {
                id: s.domain_entity_field.id,
                name: s.name,
                version: s.domain_entity_field.version,
            })
            .collect()
    }
}
