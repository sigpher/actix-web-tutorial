use std::io;

use actix_web::{
    get,
    middleware::{self, Logger},
    post, web, App, HttpResponse, HttpServer, Responder,
};
use env_logger::Env;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(Logger::default())
            // .wrap(Logger::new("%t %a"))
            .service(index)
            .service(product)
            .service(query_info)
            .service(submit)
            .service(form_submit)
            .service(login)
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

// Query
#[get("/query")]
async fn query_info(info: web::Query<Info>) -> impl Responder {
    let info = info.into_inner();
    HttpResponse::Ok().body(format!("Username :{}", info.username))
}

#[post("/submit")]
// Json
async fn submit(info: web::Json<Info>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Username :{}, Age: {}",
        info.username,
        info.age.unwrap()
    ))
}

#[post("/form_submit")]
async fn form_submit(form: web::Form<FormInfo>) -> impl Responder {
    let form = form.into_inner();

    HttpResponse::Ok().body(format!("{}: {}", form.username, form.password))
}

#[post("/login")]
async fn login(user: web::Form<User>) -> io::Result<impl Responder> {
    let user = User {
        id: user.id,
        username: user.username.to_string(),
    };
    Ok(web::Json(user))
}

#[derive(Deserialize)]
struct Product {
    category: String,
    name: String,
}

#[derive(Deserialize)]
struct Info {
    username: String,
    age: Option<u8>,
}

#[derive(Deserialize)]
struct FormInfo {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    username: String,
}
