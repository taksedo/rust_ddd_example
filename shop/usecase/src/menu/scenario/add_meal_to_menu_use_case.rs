use std::fmt::Debug;

use async_trait::async_trait;
use common::types::base::{AM, DomainEventTrait};
use derive_new::new;
use domain::menu::{
    meal::Meal,
    meal_already_exists::MealAlreadyExists,
    value_objects::{
        meal_description::MealDescription,
        meal_id::{MealId, MealIdGenerator},
        meal_name::MealName,
        price::Price,
    },
};

use crate::menu::{
    access::meal_persister::MealPersister,
    add_meal_to_menu::{AddMealToMenu, AddMealToMenuUseCaseError},
};

#[derive(new, Debug)]
pub struct AddMealToMenuUseCase {
    pub meal_persister: AM<dyn MealPersister>,
    pub id_generator: AM<dyn MealIdGenerator>,
    pub meal_exists: AM<dyn MealAlreadyExists>,
}

#[async_trait]
impl AddMealToMenu for AddMealToMenuUseCase {
    async fn execute(
        &mut self,
        name: &MealName,
        description: &MealDescription,
        price: &Price,
    ) -> Result<MealId, AddMealToMenuUseCaseError> {
        let new_meal_in_menu = Meal::add_meal_to_menu(
            self.id_generator.clone(),
            self.meal_exists.clone(),
            name.clone(),
            description.clone(),
            price.clone(),
        )
        .await?;
        self.meal_persister
            .lock()
            .await
            .save(new_meal_in_menu.clone())
            .await;
        Ok(*new_meal_in_menu.id())
    }
}

impl DomainEventTrait for AddMealToMenuUseCase {}

#[cfg(test)]
mod tests {
    use common::types::base::AMTrait;
    use domain::test_fixtures::*;

    use super::*;
    use crate::test_fixtures::MockMealPersister;

    #[tokio::test]
    async fn successfully_added() {
        let name = rnd_meal_name();
        let description = rnd_meal_description();
        let price = rnd_price();
        let id_generator = AM::new_am(TestMealIdGenerator::new());
        let meal_persister = AM::new_am(MockMealPersister::new());

        let mut add_to_menu_use_case = AddMealToMenuUseCase::new(
            meal_persister.clone(),
            id_generator.clone(),
            AM::new_am(TestMealAlreadyExists { value: false }),
        );
        let result = add_to_menu_use_case
            .execute(&name, &description, &price)
            .await;

        let id = id_generator.lock().await.id;

        assert_eq!(result.unwrap(), id.to_owned());

        meal_persister.lock().await.verify_invoked(
            Some(&id),
            Some(&name),
            Some(&description),
            Some(&price),
        );
    }

    #[tokio::test]
    async fn meal_already_exists() {
        let name = rnd_meal_name();
        let description = rnd_meal_description();
        let price = rnd_price();

        let id_generator = AM::new_am(TestMealIdGenerator::new());
        let persister = AM::new_am(MockMealPersister::new());

        let mut add_to_menu_use_case = AddMealToMenuUseCase::new(
            persister,
            id_generator,
            AM::new_am(TestMealAlreadyExists { value: true }),
        );
        let result = add_to_menu_use_case
            .execute(&name, &description, &price)
            .await;

        assert_eq!(result, Err(AddMealToMenuUseCaseError::AlreadyExists));
    }

    #[derive(new, Default, Debug, Clone, PartialEq)]
    pub struct TestMealIdGenerator {
        #[new(value = "rnd_meal_id()")]
        id: MealId,
    }

    impl MealIdGenerator for TestMealIdGenerator {
        fn generate(&mut self) -> MealId {
            self.id
        }
    }
}
