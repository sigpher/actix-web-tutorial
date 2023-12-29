use std::io;

use actix_web::{Responder, HttpResponse, get, post, HttpServer, App, web};

#[tokio::main]
async fn main()->io::Result<()> {
    HttpServer::new(||{
        App::new()
        .service(hello)
        .service(echo)
        .route("/hey",web::get().to(manual_hello))
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}


#[get("/")]
async fn hello()->impl Responder{
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body:String)->impl Responder{
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello()->impl Responder{
    HttpResponse::Ok().body("Hey there!")
}