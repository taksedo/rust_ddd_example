use std::sync::{Arc, Mutex};

use derive_new::new;
use domain::menu::value_objects::meal_id::MealId;

use crate::menu::{
    access::meal_extractor::MealExtractor,
    dto::meal_info::MealInfo,
    get_meal_by_id::{GetMealById, GetMealByIdUseCaseError},
};

#[derive(new, Debug)]
pub struct GetMealByIdUseCase {
    pub meal_extractor: Arc<Mutex<dyn MealExtractor>>,
}

impl GetMealById for GetMealByIdUseCase {
    fn execute(&mut self, id: &MealId) -> Result<MealInfo, GetMealByIdUseCaseError> {
        match self.meal_extractor.lock().unwrap().get_by_id(id) {
            res if res.is_some() && res.clone().unwrap().visible() => {
                let res = res.unwrap();
                Ok(MealInfo::from(res))
            }
            _ => Err(GetMealByIdUseCaseError::MealNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use domain::test_fixtures::{rnd_meal, rnd_meal_id};

    use super::*;
    use crate::test_fixtures::{removed_meal, MockMealExtractor};

    #[test]
    fn meal_not_found() {
        let meal_extractor = Arc::new(Mutex::new(MockMealExtractor::new()));
        let mut use_case = GetMealByIdUseCase::new(meal_extractor);

        let meal_id = &rnd_meal_id();
        let result = use_case.execute(meal_id);

        assert_eq!(result, Err(GetMealByIdUseCaseError::MealNotFound));
        use_case
            .meal_extractor
            .lock()
            .unwrap()
            .downcast_ref::<MockMealExtractor>()
            .unwrap()
            .verify_invoked_get_by_id(&meal_id);
    }

    #[test]
    fn meal_removed() {
        let meal = removed_meal();
        let meal_extractor = Arc::new(Mutex::new(MockMealExtractor {
            meal: Option::from(meal.to_owned()),
            ..MockMealExtractor::default()
        }));

        let mut use_case = GetMealByIdUseCase::new(meal_extractor);
        let result = use_case.execute(meal.id());

        assert_eq!(result, Err(GetMealByIdUseCaseError::MealNotFound));
        use_case
            .meal_extractor
            .lock()
            .unwrap()
            .downcast_ref::<MockMealExtractor>()
            .unwrap()
            .verify_invoked_get_by_id(meal.id());
    }

    #[test]
    fn meal_extracted_successfully() {
        let meal = rnd_meal();
        let meal_extractor = Arc::new(Mutex::new(MockMealExtractor {
            meal: Option::from(meal.to_owned()),
            ..MockMealExtractor::default()
        }));
        let mut use_case = GetMealByIdUseCase::new(meal_extractor);

        let result = use_case.execute(meal.id());
        let meal_info = result;

        assert_eq!(
            meal_info.unwrap(),
            MealInfo {
                id: *meal.id(),
                name: meal.name().to_owned(),
                description: meal.description().to_owned(),
                price: meal.price().to_owned(),
                version: *meal.version(),
            }
        );
        use_case
            .meal_extractor
            .lock()
            .unwrap()
            .downcast_ref::<MockMealExtractor>()
            .unwrap()
            .verify_invoked_get_by_id(&meal.id());
    }
}
