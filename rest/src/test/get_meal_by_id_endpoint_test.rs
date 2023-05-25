use crate::main::menu::add_meal_to_menu_endpoint::AddMealToMenuEndpointSharedState;
use crate::test_fixtures::fixtures::MockGetMealById;
use actix_web::web;
use std::sync::Arc;
use usecase::main::menu::get_meal_by_id::GetMealByIdUseCaseError;

#[actix_web::test]
async fn meal_not_found() {
    todo!();
    // let mut get_meal_by_id = MockGetMealById::default();
    // get_meal_by_id.response = Err(GetMealByIdUseCaseError::MealNotFound);
    // let mock_shared_state = web::Data::new(AddMealToMenuEndPointSharedState {
    //     add_meal_to_menu: Arc::clone(&mock_add_meal_to_menu),
    // });
}
