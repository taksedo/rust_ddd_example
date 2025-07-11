use std::fmt::Debug;

use actix_web::{HttpResponse, http::header::ContentType, web};
use common::types::base::AM;
use usecase::menu::GetMenu;

use crate::{endpoint_url::API_V1_MENU_GET_ALL, menu::meal_model::MealModel};

/// Get the menu
#[utoipa::path(
    get,
    path = API_V1_MENU_GET_ALL,
    tag = "Meal",
    responses(
        (
            status = OK,
            body = Vec<MealModel>,
            description = "OK" 
        )
    )
)]

pub async fn get_menu_endpoint<T: GetMenu + Send + Debug>(
    shared_state: web::Data<AM<T>>,
) -> HttpResponse {
    let meal_model_list: Vec<MealModel> = shared_state
        .lock()
        .await
        .execute()
        .await
        .into_iter()
        .map(MealModel::from)
        .collect();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&meal_model_list).unwrap())
}

pub fn get_menu_endpoint_config<T: GetMenu + Send + Debug + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.route(API_V1_MENU_GET_ALL, web::get().to(get_menu_endpoint::<T>));
}

#[cfg(test)]
mod tests {
    use actix_web::body::MessageBody;
    use common::types::base::AMTrait;

    use super::*;
    use crate::test_fixtures::{MockGetMenu, rnd_meal_info};

    #[actix_web::test]
    async fn get_menu() {
        let meal_info = rnd_meal_info();
        let mock_get_menu = AM::new_am(MockGetMenu::default());
        mock_get_menu.lock().await.meal_info = meal_info.clone();
        let mock_shared_state = web::Data::new(mock_get_menu.clone());

        let resp = get_menu_endpoint(mock_shared_state).await;

        let body = resp.into_body().try_into_bytes().unwrap();
        let body_text = std::str::from_utf8(&body).unwrap();

        let list_of_meal_model_from_resp: Vec<MealModel> = serde_json::from_str(body_text).unwrap();

        assert_eq!(list_of_meal_model_from_resp.len(), 1);
        assert_eq!(
            list_of_meal_model_from_resp.first().unwrap().id,
            meal_info.id.to_i64()
        );
        assert_eq!(
            list_of_meal_model_from_resp.first().unwrap().name,
            meal_info.name.to_string()
        );
        assert_eq!(
            list_of_meal_model_from_resp.first().unwrap().description,
            meal_info.description.to_string()
        );
        assert_eq!(
            list_of_meal_model_from_resp.first().unwrap().price,
            meal_info.price.to_bigdecimal()
        );
        assert_eq!(
            list_of_meal_model_from_resp.first().unwrap().version,
            meal_info.version.to_i64()
        );
    }
}
