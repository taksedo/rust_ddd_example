use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};
use common::common_rest::rest_responses::{
    get_json_from_http_response, resource_not_found, to_invalid_param_bad_request,
    GenericErrorResponse,
};
use domain::menu::value_objects::meal_id::MealId;
use usecase::menu::{
    get_meal_by_id::{GetMealById, GetMealByIdUseCaseError},
    scenario::get_meal_by_id_use_case::GetMealByIdUseCase,
};

use crate::{
    endpoint_url::API_V1_MENU_GET_BY_ID, menu::meal_model::MealModel, to_error::ToRestError,
    validated::Validated,
};

/// Get a meal by id
#[utoipa::path(
    get,
    path = API_V1_MENU_GET_BY_ID,
    tag = "Meal", 
    params(
        (
            "id" = i64,
            Path,
            description = "Meal id"
        )
    ),
    responses(
        (
            status = NOT_FOUND,
            description = "Meal not found",
            body = GenericErrorResponse,
            example = json!(&(get_json_from_http_response(resource_not_found())))
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
    )
)]
pub async fn get_meal_by_id_endpoint<T: GetMealById + Send + Debug>(
    shared_state: web::Data<Arc<Mutex<T>>>,
    req: HttpRequest,
) -> HttpResponse {
    let id: i64 = req.match_info().get("id").unwrap().parse().unwrap();

    let error_list = Arc::new(Mutex::new(vec![]));

    match MealId::validated(id, error_list.clone()) {
        Ok(meal_id) => match shared_state.lock().unwrap().execute(&meal_id) {
            Ok(meal_info) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&MealModel::from(meal_info)).unwrap()),
            Err(e) => e.to_rest_error(),
        },
        Err(_) => to_invalid_param_bad_request(error_list),
    }
}

impl ToRestError for GetMealByIdUseCaseError {
    fn to_rest_error(self) -> HttpResponse {
        resource_not_found()
    }
}

pub fn get_meal_by_id_endpoint_config(cfg: &mut web::ServiceConfig) {
    cfg.route(
        API_V1_MENU_GET_BY_ID,
        web::get().to(get_meal_by_id_endpoint::<GetMealByIdUseCase>),
    );
}

#[cfg(test)]
mod tests {
    use actix_web::{body::MessageBody, http::StatusCode, test::TestRequest, web::Data};
    use common::common_rest::rest_responses::{not_found_type_url, GenericErrorResponse};
    use dotenvy::dotenv;
    use usecase::menu::get_meal_by_id::GetMealByIdUseCaseError::MealNotFound;

    use super::*;
    use crate::{
        domain_test_fixtures::rnd_meal_id,
        test_fixtures::{rnd_meal_info, MockGetMealById},
    };

    #[actix_web::test]
    async fn returned_successfully() {
        let meal_info = rnd_meal_info();

        let mock_get_meal_by_id = mock_get_meal_by_id();
        let mock_shared_state = mock_shared_state(&mock_get_meal_by_id);

        mock_get_meal_by_id.lock().unwrap().response = Ok(meal_info.clone());

        let req = TestRequest::default()
            .param("id", meal_info.id.to_i64().to_string())
            .to_http_request();

        let resp = get_meal_by_id_endpoint(mock_shared_state, req).await;

        let body = resp.into_body().try_into_bytes().unwrap();
        let body_json = std::str::from_utf8(&body).unwrap();

        let meal_info = MealModel::from(meal_info);
        let meal_info_json = serde_json::to_string(&meal_info).unwrap();
        assert_eq!(body_json, &meal_info_json);

        mock_get_meal_by_id
            .lock()
            .unwrap()
            .verify_invoked(&MealId::try_from(meal_info.id).unwrap());
    }

    #[actix_web::test]
    async fn meal_not_found() {
        dotenv().ok();
        let mock_get_meal_by_id = mock_get_meal_by_id();
        let mock_shared_state = mock_shared_state(&mock_get_meal_by_id);

        mock_get_meal_by_id.lock().unwrap().response = Err(MealNotFound);

        let meal_id = rnd_meal_id().to_i64();

        let req = TestRequest::default()
            .param("id", meal_id.to_string())
            .to_http_request();

        let resp = get_meal_by_id_endpoint(mock_shared_state, req).await;

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

    fn mock_get_meal_by_id() -> Arc<Mutex<MockGetMealById>> {
        Arc::new(Mutex::new(MockGetMealById::default()))
    }

    fn mock_shared_state(
        mock_get_meal_by_id: &Arc<Mutex<MockGetMealById>>,
    ) -> Data<Arc<Mutex<MockGetMealById>>> {
        Data::new(Arc::clone(mock_get_meal_by_id))
    }
}
