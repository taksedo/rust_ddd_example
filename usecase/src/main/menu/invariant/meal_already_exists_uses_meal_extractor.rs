use crate::main::menu::access::meal_extractor::MealExtractor;
use derive_new::new;
use domain::main::menu::meal_already_exists::MealAlreadyExists;
use domain::main::menu::meal_name::MealName;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(new, Debug, Clone)]
pub struct MealAlreadyExistsUsesMealExtractor {
    pub extractor: Rc<RefCell<dyn MealExtractor>>,
}

impl MealAlreadyExists for MealAlreadyExistsUsesMealExtractor {
    fn invoke(&mut self, name: &MealName) -> bool {
        let meal_found_by_get = self.extractor.borrow_mut().get_by_name(name.to_owned());
        let meal_found_by_get_is_removed = meal_found_by_get.clone().unwrap_or_default().removed;
        meal_found_by_get.is_some() & !meal_found_by_get_is_removed
    }
}