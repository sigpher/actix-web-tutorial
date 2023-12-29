use actix_web::web;

use crate::module::counter::AppStateWithCounter;

pub async fn get_counter(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter +=1;
    format!("Request number: {counter}")
}
