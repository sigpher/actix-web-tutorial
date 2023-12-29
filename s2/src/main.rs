use std::io;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[tokio::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(product)
            .service(query_info)
            .route("/goods/{id}", web::get().to(goods))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Path I
#[get("/users/{user_id}/{friend}")]
async fn index(path: web::Path<(u32, String)>) -> impl Responder {
    let (user_id, friend) = path.into_inner();
    HttpResponse::Ok().body(format!("Welcome {}, user ID: {}", friend, user_id))
}

// Path II
async fn goods(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("Goods ID: {}", id))
}

// Path III
#[get("/product/{category}/{name}")]
async fn product(product: web::Path<Product>) -> impl Responder {
    let p = product.into_inner();
    HttpResponse::Ok().body(format!("Product Category: {} name: {}", p.category, p.name))
}

#[get("/query")]
async fn query_info(info: web::Query<Info>) -> impl Responder {
    let info = info.into_inner();
    HttpResponse::Ok().body(format!("Username :{}", info.username))
}

#[derive(Deserialize)]
struct Product {
    category: String,
    name: String,
}

#[derive(Deserialize)]
struct Info {
    username: String,
}
