use std::fmt::Debug;

use actix_web::{HttpResponse, http, web};
use bigdecimal::BigDecimal;
use common::{
    common_rest::{
        GenericErrorResponse, created, rest_business_error, to_invalid_param_bad_request,
    },
    types::base::{AM, RCell, RcRefCellTrait},
};
use derive_new::new;
use domain::menu::value_objects::{
    meal_description::MealDescription, meal_name::MealName, price::Price,
};
use http::Uri;
use serde::{Deserialize, Serialize};
use usecase::menu::{AddMealToMenu, AddMealToMenuUseCaseError};
use utoipa::ToSchema;

use crate::{
    endpoint_url::{API_V1_MENU_ADD_TO_MENU, API_V1_MENU_GET_BY_ID},
    to_error::ToRestError,
    validated::Validated,
};

#[derive(new, Serialize, Deserialize, Debug, ToSchema)]
pub struct AddMealToMenuRestRequest {
    /// Name of the meal
    #[schema(example = "Шаурма средняя", required = true)]
    name: String,
    /// Description of the meal
    #[schema(example = "Средний размер, средняя цена", required = true)]
    description: String,
    /// Price of the meal
    #[schema(example = 200, required = true)]
    price: f64,
}

/// Add a meal to the menu
#[utoipa::path(
    post,
    path = API_V1_MENU_ADD_TO_MENU,
    tag = "Meal",
    request_body(
        content = AddMealToMenuRestRequest,
    ),
    responses(
        (
            status = CREATED,
            description = "Created",
            headers(
                ("location" = String, description = "Location of new Meal")
            )
        ),
        (
            status = BAD_REQUEST,
            description = "Bad request",
            body = GenericErrorResponse,
            example = json!(
                {
                    "type":"http://0.0.0.0:8080/bad_request",
                    "title":"Bad request",
                    "status":400,
                    "invalid_params":
                    [
                        {"message":"Meal name is empty."},
                        {"message":"Meal description is empty"},
                        {"message":"Price scale must not be > 2"}
                    ]
                }
            )
        ),
        (
            status = UNPROCESSABLE_ENTITY,
            description = "Meal already exists",
            body = GenericErrorResponse,
            example = json!({"type":"http://0.0.0.0:8080/already_exists","title":"Meal already exists","status":422})
        ),
    ))]
pub async fn add_meal_to_menu_endpoint<T>(
    shared_state: web::Data<AM<T>>,
    request: web::Json<AddMealToMenuRestRequest>,
) -> HttpResponse
where
    T: AddMealToMenu + Send + Debug,
{
    println!("Request {request:?} to add meal to menu received");

    let error_list = RCell::new_rc(vec![]);

    let meal_name = MealName::validated(&request.name, error_list.clone());
    let meal_description = MealDescription::validated(&request.description, error_list.clone());
    let price = request
        .price
        .to_string()
        .parse::<BigDecimal>()
        .ok()
        .and_then(|p| Price::validated(p, error_list.clone()));

    match (meal_name, meal_description, price) {
        (Some(meal_name), Some(meal_description), Some(price)) => {
            match shared_state
                .lock()
                .await
                .execute(&meal_name, &meal_description, &price)
                .await
            {
                Ok(meal_id) => created(
                    API_V1_MENU_GET_BY_ID
                        .replace("{id}", &meal_id.to_i64().to_string())
                        .parse::<Uri>()
                        .unwrap(),
                ),
                Err(e) => e.to_rest_error(),
            }
        }
        _ => to_invalid_param_bad_request(error_list),
    }
}

impl ToRestError for AddMealToMenuUseCaseError {
    fn to_rest_error(self) -> HttpResponse {
        rest_business_error("Meal already exists", "already_exists")
    }
}

pub fn add_meal_to_menu_endpoint_config<T>(cfg: &mut web::ServiceConfig)
where
    T: AddMealToMenu + Send + Debug + 'static,
{
    cfg.route(
        API_V1_MENU_ADD_TO_MENU,
        web::post().to(add_meal_to_menu_endpoint::<T>),
    );
}

#[cfg(test)]
mod tests {
    use actix_web::{
        body::MessageBody,
        http::{StatusCode, header},
        web::{Data, Json},
    };
    use bigdecimal::{ToPrimitive, num_bigint::BigInt};
    use common::{
        common_rest::{bad_request_type_url, error_type_url},
        types::base::AMTrait,
    };
    use domain::test_fixtures::*;
    use dotenvy::dotenv;

    use super::*;
    use crate::test_fixtures::{MockAddMealToMenu, StringMethodsForRestTestExt};

    #[actix_web::test]
    async fn created_successfully() {
        dotenv().ok();
        let meal_id = rnd_meal_id();
        let meal_name = rnd_meal_name();
        let meal_description = rnd_meal_description();
        let price = rnd_price();

        let mock_add_meal_to_menu = mock_add_meal_to_menu();
        mock_add_meal_to_menu.lock().await.response = Ok(meal_id);

        let mock_shared_state = mock_shared_state(&mock_add_meal_to_menu);

        let meal = Json(AddMealToMenuRestRequest::new(
            meal_name.clone().to_string(),
            meal_description.clone().to_string(),
            price.to_bigdecimal().to_f64().unwrap(),
        ));

        let resp = add_meal_to_menu_endpoint(mock_shared_state, meal).await;

        mock_add_meal_to_menu
            .lock()
            .await
            .verify_invoked(&meal_name, &meal_description, &price);

        let header = resp
            .headers()
            .get(header::LOCATION)
            .unwrap()
            .to_str()
            .unwrap();

        assert_eq!(&resp.status(), &StatusCode::CREATED);
        assert_eq!(
            header,
            API_V1_MENU_GET_BY_ID.to_string().with_id(&meal_id.to_i64())
        );
    }

    #[actix_web::test]
    async fn validation_error() {
        dotenv().ok();
        let mock_add_meal_to_menu = mock_add_meal_to_menu();
        let mock_shared_state = mock_shared_state(&mock_add_meal_to_menu);

        let meal = Json(AddMealToMenuRestRequest::new(
            "".to_string(),
            "".to_string(),
            BigDecimal::new(BigInt::from(1), 20).to_f64().unwrap(),
        ));

        let resp = add_meal_to_menu_endpoint(mock_shared_state, meal).await;
        let body = resp.into_body().try_into_bytes().unwrap();
        let body_text = std::str::from_utf8(&body).unwrap();

        let response_dto: GenericErrorResponse = serde_json::from_str(body_text).unwrap();

        assert_eq!(
            &response_dto.response_status,
            &StatusCode::BAD_REQUEST.as_u16()
        );
        assert_eq!(&response_dto.response_type, &bad_request_type_url());
        assert_eq!(
            &response_dto.response_status,
            &StatusCode::BAD_REQUEST.as_u16()
        );
        assert_eq!(&response_dto.response_title, &"Bad request");
        assert_eq!(response_dto.invalid_params.len(), 3);
    }

    #[actix_web::test]
    async fn meal_already_exists() {
        dotenv().ok();
        let mock_add_meal_to_menu = mock_add_meal_to_menu();
        let mock_shared_state = mock_shared_state(&mock_add_meal_to_menu);
        mock_add_meal_to_menu.lock().await.response = Err(AddMealToMenuUseCaseError::AlreadyExists);
        let meal = Json(AddMealToMenuRestRequest::new(
            rnd_meal_name().to_string(),
            rnd_meal_description().to_string(),
            rnd_price().to_f64(),
        ));

        let resp = add_meal_to_menu_endpoint(mock_shared_state, meal).await;

        assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);

        let body = resp.into_body().try_into_bytes().unwrap();
        let body_text = std::str::from_utf8(&body).unwrap();

        let response_dto: GenericErrorResponse = serde_json::from_str(body_text).unwrap();

        assert_eq!(
            &response_dto.response_status,
            &StatusCode::UNPROCESSABLE_ENTITY.as_u16()
        );
        assert_eq!(
            &response_dto.response_type,
            &error_type_url("already_exists")
        );
        assert_eq!(
            &response_dto.response_status,
            &StatusCode::UNPROCESSABLE_ENTITY.as_u16()
        );
        assert_eq!(&response_dto.response_title, "Meal already exists");
    }

    fn mock_shared_state(
        mock_add_meal_to_menu: &AM<MockAddMealToMenu>,
    ) -> Data<AM<MockAddMealToMenu>> {
        Data::new(mock_add_meal_to_menu.clone())
    }

    fn mock_add_meal_to_menu() -> AM<MockAddMealToMenu> {
        AM::new_am(MockAddMealToMenu::default())
    }
}
