use std::sync::{Arc, Mutex};

use derive_new::new;

use domain::main::menu::value_objects::meal_id::MealId;

use crate::main::menu::access::meal_extractor::MealExtractor;
use crate::main::menu::dto::meal_info::MealInfo;
use crate::main::menu::get_meal_by_id::{GetMealById, GetMealByIdUseCaseError};

#[derive(new, Debug)]
pub struct GetMealByIdUseCase {
    pub meal_extractor: Arc<Mutex<dyn MealExtractor>>,
}

impl GetMealById for GetMealByIdUseCase {
    fn execute(&mut self, id: MealId) -> Result<MealInfo, GetMealByIdUseCaseError> {
        match self.meal_extractor.lock().unwrap().get_by_id(id) {
            res if res.is_some() && res.clone().unwrap().visible() => {
                let res = res.unwrap();
                Ok(MealInfo::from(res))
            }
            _ => Err(GetMealByIdUseCaseError::MealNotFound),
        }
    }
}
