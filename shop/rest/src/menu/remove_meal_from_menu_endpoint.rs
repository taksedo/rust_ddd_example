use std::fmt::Debug;

use actix_web::{HttpRequest, HttpResponse, http::StatusCode, web};
use common::{
    common_rest::{
        GenericErrorResponse, get_json_from_http_response, resource_not_found,
        to_invalid_param_bad_request,
    },
    types::base::{AM, ArcMutexTrait, RCell},
};
use domain::menu::value_objects::meal_id::MealId;
use usecase::menu::{RemoveMealFromMenu, RemoveMealFromMenuUseCaseError};

use crate::{endpoint_url::API_V1_MENU_DELETE_BY_ID, to_error::ToRestError, validated::Validated};

/// Remove the meal from the menu
#[utoipa::path(
    delete,
    path = API_V1_MENU_DELETE_BY_ID,
    tag = "Meal",
    params(
        ("id" = i64, Path,  description = "Meal id")
    ),
    responses(
        (
            status = NO_CONTENT,
            description = "Meal successfully removed" 
        ),
        (
            status = BAD_REQUEST,
            description = "Invalid id",
            body = GenericErrorResponse,
            example = json!(
                {
                    "type": "http://0.0.0.0:8080/bad_request",
                    "title": "Bad request",
                    "status": 400,
                    "invalid_params": 
                    [
                        {"message": "Meal Id must be > 0"}
                    ]
                }
            )
        ),
        (
            status = NOT_FOUND,
            description = "Meal not found",
            body = GenericErrorResponse,
            example = json!(&(get_json_from_http_response(resource_not_found())))
        )
    ))]
pub async fn remove_meal_from_menu_endpoint<T>(
    shared_state: web::Data<AM<T>>,
    req: HttpRequest,
) -> HttpResponse
where
    T: RemoveMealFromMenu + Send + Debug,
{
    let id: i64 = req.match_info().get("id").unwrap().parse().unwrap();

    let error_list = RCell::new_rc(vec![]);

    match MealId::validated(id, error_list.clone()) {
        Ok(meal_id) => match shared_state.lock_un().execute(&meal_id) {
            Ok(_) => HttpResponse::new(StatusCode::NO_CONTENT),
            Err(e) => e.to_rest_error(),
        },
        Err(_) => to_invalid_param_bad_request(error_list),
    }
}

impl ToRestError for RemoveMealFromMenuUseCaseError {
    fn to_rest_error(self) -> HttpResponse {
        resource_not_found()
    }
}

pub fn remove_meal_from_menu_endpoint_config<T>(cfg: &mut web::ServiceConfig)
where
    T: RemoveMealFromMenu + Send + Debug + 'static,
{
    cfg.route(
        API_V1_MENU_DELETE_BY_ID,
        web::delete().to(remove_meal_from_menu_endpoint::<T>),
    );
}

#[cfg(test)]
mod tests {
    use actix_web::{body::MessageBody, test::TestRequest, web::Data};
    use common::common_rest::{GenericErrorResponse, not_found_type_url};
    use domain::test_fixtures::*;
    use dotenvy::dotenv;

    use super::*;
    use crate::test_fixtures::MockRemoveMealFromMenu;
    #[actix_web::test]
    async fn meal_not_found() {
        dotenv().ok();
        let meal_id = rnd_meal_id();
        let mock_remove_meal_from_menu = AM::new_am(MockRemoveMealFromMenu::default());
        mock_remove_meal_from_menu.lock_un().response =
            Err(RemoveMealFromMenuUseCaseError::MealNotFound);
        let mock_shared_state = Data::new(mock_remove_meal_from_menu.clone());

        let req = TestRequest::default()
            .param("id", meal_id.to_i64().to_string())
            .to_http_request();

        let resp = remove_meal_from_menu_endpoint(mock_shared_state, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        let body = resp.into_body().try_into_bytes().unwrap();
        let body_text = std::str::from_utf8(&body).unwrap();

        let response_dto: GenericErrorResponse = serde_json::from_str(body_text).unwrap();

        assert_eq!(&response_dto.response_type, &not_found_type_url());
        assert_eq!(
            &response_dto.response_status,
            &StatusCode::NOT_FOUND.as_u16()
        );
        assert_eq!(&response_dto.response_title, "Resource not found");
    }

    #[actix_web::test]
    async fn removed_successfully() {
        let meal_id = rnd_meal_id();

        let mock_remove_meal_from_menu = AM::new_am(MockRemoveMealFromMenu::default());
        let mock_shared_state = Data::new(mock_remove_meal_from_menu.clone());

        let req = TestRequest::default()
            .param("id", meal_id.to_i64().to_string())
            .to_http_request();

        let resp = remove_meal_from_menu_endpoint(mock_shared_state, req).await;

        let body = resp.into_body().try_into_bytes().unwrap();

        assert!(body.is_empty());
    }
}
