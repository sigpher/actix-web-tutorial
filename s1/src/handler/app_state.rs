use actix_web::{get, web};

use crate::module::AppState;

#[get("/app_state")]
pub async fn get_state(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}")
}
