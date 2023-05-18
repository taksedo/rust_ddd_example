use crate::main::menu::access::meal_extractor::MealExtractor;
use derive_new::new;
use domain::main::menu::meal_already_exists::MealAlreadyExists;
use domain::main::menu::meal_name::MealName;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(new, Debug)]
pub struct MealAlreadyExistsImpl {
    extractor: Rc<RefCell<dyn MealExtractor>>,
}

impl MealAlreadyExists for MealAlreadyExistsImpl {
    fn invoke(&mut self, name: &MealName) -> bool {
        let meal = &self.extractor.borrow_mut().get_by_name(name.clone());
        meal.is_some() && !meal.clone().unwrap().removed
    }
}
