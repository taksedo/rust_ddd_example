use crate::main::menu::access::meal_extractor::MealExtractor;
use crate::main::menu::dto::meal_info::MealInfo;
use crate::main::menu::get_meal_by_id::{GetMealById, GetMealByIdUseCaseError};
use derive_new::new;
use domain::main::menu::meal_id::MealId;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(new, Debug)]
pub struct GetMealByIdUseCase {
    pub meal_extractor: Rc<RefCell<dyn MealExtractor>>,
}

impl GetMealById for GetMealByIdUseCase {
    fn execute(&mut self, id: MealId) -> Result<MealInfo, GetMealByIdUseCaseError> {
        self.meal_extractor.borrow_mut().get_by_id(id);
        // dbg!(&self.meal_extractor);
        match self.meal_extractor.borrow_mut().get_by_id(id) {
            res if res.is_some() && res.clone().unwrap().visible() => {
                let res = res.unwrap();
                Ok(MealInfo {
                    id: res.domain_entity_field.id,
                    name: res.clone().name,
                    // description: res.to_owned().unwrap().description,
                    // price: res.to_owned().unwrap().price,
                    version: res.domain_entity_field.version,
                })
            }
            _ => Err(GetMealByIdUseCaseError::MealNotFound),
        }
    }
}
