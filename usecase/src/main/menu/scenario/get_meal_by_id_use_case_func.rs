use crate::main::menu::access::meal_extractor::MealExtractor;
use crate::main::menu::dto::meal_info::MealInfo;
use crate::main::menu::get_meal_by_id::GetMealByIdUseCaseError;
use domain::main::menu::meal_id::MealId;
use std::sync::{Arc, Mutex};

pub fn get_meal_by_id_use_case(
    meal_extractor: Arc<Mutex<impl MealExtractor>>,
    id: MealId,
) -> Result<MealInfo, GetMealByIdUseCaseError> {
    meal_extractor.lock().unwrap().get_by_id(id);

    match meal_extractor.lock().unwrap().get_by_id(id) {
        res if res.is_some() && res.clone().unwrap().visible() => {
            let res = res.unwrap();
            Ok(MealInfo {
                id: res.domain_entity_field.id,
                name: res.clone().name,
                description: res.to_owned().description,
                price: res.to_owned().price,
                version: res.domain_entity_field.version,
            })
        }
        _ => Err(GetMealByIdUseCaseError::MealNotFound),
    }
}