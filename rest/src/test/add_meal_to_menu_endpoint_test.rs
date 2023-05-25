use crate::main::menu::add_meal_to_menu_endpoint;
use crate::main::menu::add_meal_to_menu_endpoint::{AddMealToMenuEndpointSharedState, MealStruct};
use crate::test_fixtures::fixtures::MockAddMealToMenu;
use actix_web::http::{header, StatusCode};
use actix_web::{web, web::Json};
use domain::test_fixtures::fixtures::{rnd_meal_id, rnd_meal_name};
use std::sync::{Arc, Mutex};

// #[actix_web::test]
// async fn validation_error() {
//     let mock_shared_state = meal_create_shared_state();
//
//     let mock_add_meal_to_menu = MockAddMealToMenu::default();
//     let mock_shared_state = web::Data::new(AddMealToMenuEndPointSharedState {
//         add_meal_to_menu: mock_shared_state,
//     });
//
//     let meal = Json(MealStruct::new("Вкусный кофе".to_string()));
//     let resp = add_meal_to_menu_endpoint::execute(mock_shared_state, meal).await;
//
//     assert_eq!(resp, "==========MealId { value: 1 }=========".to_string());
// }

#[actix_web::test]
async fn created_successfully() {
    let meal_id = rnd_meal_id();
    let meal_name = rnd_meal_name();

    let mock_add_meal_to_menu = Arc::new(Mutex::new(MockAddMealToMenu::default()));
    mock_add_meal_to_menu.lock().unwrap().response = Ok(meal_id);

    let mock_shared_state = web::Data::new(AddMealToMenuEndpointSharedState {
        add_meal_to_menu: Arc::clone(&mock_add_meal_to_menu),
    });

    let meal = Json(MealStruct::new(meal_name.clone().value));

    let resp = add_meal_to_menu_endpoint::execute(mock_shared_state, meal).await;

    mock_add_meal_to_menu
        .lock()
        .unwrap()
        .verify_invoked(meal_name);
    let resp = resp.unwrap();

    let header = resp
        .headers()
        .get(header::LOCATION)
        .unwrap()
        .to_str()
        .unwrap();

    println!("{:?}", &resp);

    assert_eq!(&resp.status(), &StatusCode::OK);
    assert_eq!(header, &meal_id.to_u64().to_string());
}
// #[actix_web::test]
// async fn test_index_not_ok() {
//     let req = test::TestRequest::default().to_http_request();
//     let resp = index(req).await;
//     assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
// }
//
// @WebMvcTest
// @ContextConfiguration(classes = [AddMealToMenuEndpointTest.TestConfiguration::class])
// internal class AddMealToMenuEndpointTest {
//
// @Autowired
// lateinit var mockMvc: MockMvc
//
// @Autowired
// lateinit var mockAddMealToMenu: MockAddMealToMenu
//
// private val mapper = ObjectMapper()
//
// #[test]
// fn validation_error() {
// mockMvc
// .post(API_V1_MENU_ADD_TO_MENU) {
// contentType = MediaType.APPLICATION_JSON
// content =
// mapper.writeValueAsString(
// AddMealToMenuRestRequest(
// name = "",
// description = "",
// price = BigDecimal.ONE.setScale(20)
// )
// )
// }.andExpect {
// content {
// contentType(MediaType.APPLICATION_PROBLEM_JSON)
// status { isBadRequest() }
// content {
// jsonPath("$.type") { value(badRequestTypeUrl()) }
// jsonPath("$.status") { value(HttpStatus.BAD_REQUEST.value()) }
// jsonPath("$.invalid_params.length()") { value(3) }
// }
// }
// }
// }
//
// @Test
// fun `meal already exists`() {
// mockAddMealToMenu.response = AddMealToMenuUseCaseError.AlreadyExists.left()
//
// val name = mealName()
// val description = mealDescription()
// val price = price()
//
// mockMvc
// .post(API_V1_MENU_ADD_TO_MENU) {
// contentType = MediaType.APPLICATION_JSON
// content =
// mapper.writeValueAsString(
// AddMealToMenuRestRequest(
// name = name.toStringValue(),
// description = description.toStringValue(),
// price = price.toBigDecimalValue()
// )
// )
// }.andExpect {
// content {
// contentType(MediaType.APPLICATION_PROBLEM_JSON)
// status { isUnprocessableEntity() }
// content {
// jsonPath("$.type") { value(errorTypeUrl("already_exists")) }
// jsonPath("$.status") { value(HttpStatus.UNPROCESSABLE_ENTITY.value()) }
// }
// }
// }
//
// mockAddMealToMenu.verifyInvoked(name, description, price)
// }
//
// @Test
// fun `created successfully`() {
// val mealId = mealId()
// mockAddMealToMenu.response = mealId.right()
//
// val name = mealName()
// val description = mealDescription()
// val price = price()
//
// val url = API_V1_MENU_ADD_TO_MENU
//
// mockMvc
// .post(url) {
// contentType = MediaType.APPLICATION_JSON
// content =
// mapper.writeValueAsString(
// AddMealToMenuRestRequest(
// name = name.toStringValue(),
// description = description.toStringValue(),
// price = price.toBigDecimalValue()
// )
// )
// }.andExpect {
// content {
// status { isCreated() }
// content {
// string("")
// }
// header { string("Location", API_V1_MENU_GET_BY_ID.withId(mealId.toLongValue()).withHost()) }
// }
// }
// mockAddMealToMenu.verifyInvoked(name, description, price)
// }
//
// @Configuration
// class TestConfiguration {
//
// @Bean
// fun addMealToMenuEndpoint(addMealToMenu: AddMealToMenu) = AddMealToMenuEndpoint(addMealToMenu)
//
// @Bean
// fun addMealToMenu() = MockAddMealToMenu()
// }
// }
