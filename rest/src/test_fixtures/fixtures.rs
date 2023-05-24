use domain::main::menu::meal_id::MealId;
use domain::main::menu::meal_name::MealName;
use domain::test_fixtures::fixtures::{rnd_meal_id, rnd_meal_name};
use usecase::main::menu::add_meal_to_menu::{AddMealToMenu, AddMealToMenuUseCaseError};

#[derive(Debug)]
pub struct MockAddMealToMenu {
    pub(crate) response: Result<MealId, AddMealToMenuUseCaseError>,
    name: MealName, // lateinit var description: MealDescription
                    // lateinit var price: Price
}

impl Default for MockAddMealToMenu {
    fn default() -> Self {
        Self {
            response: Ok(rnd_meal_id()),
            name: rnd_meal_name(),
        }
    }
}

// impl From<MockAddMealToMenu> for AddMealToMenuUseCase {
//     fn from(value: MockAddMealToMenu) -> Self {
//         Self {
//             meal_persister: Arc::new(Mutex::new(())),
//             id_generator: Arc::new(Mutex::new(())),
//             meal_exists: Arc::new(Mutex::new(())),
//         }
//     }
// }

impl AddMealToMenu for MockAddMealToMenu {
    fn execute(
        &mut self,
        name: MealName,
        // description: MealDescription,
        // price: Price,
    ) -> Result<MealId, AddMealToMenuUseCaseError> {
        self.name = name;
        // self.description.clone() = description
        // this.price = price
        self.response.to_owned()
    }
}

impl MockAddMealToMenu {
    pub fn verify_invoked(
        &self,
        name: MealName,
        // description: MealDescription,
        // price: Price,
    ) {
        assert_eq!(name, self.name.clone());
        // description shouldBe this.description
        // price shouldBe this.price
    }
}
