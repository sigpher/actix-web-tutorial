use actix_web::{get,HttpResponse, Responder};

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
