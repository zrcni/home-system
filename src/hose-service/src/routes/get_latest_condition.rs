use actix_web::{Error, HttpResponse, web};

use crate::startup::AppState;

pub async fn get_latest_condition(
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let device_id = path.into_inner();
    let latest_condition = state
        .conditions_repo
        .find_latest(&device_id)
        .await
        .expect("Failed to find latest condition");
    Ok(HttpResponse::Ok().json(latest_condition))
}
