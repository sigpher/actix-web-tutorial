use std::io;
use actix_web::{ web, App, HttpServer};
pub mod handler;

#[tokio::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/greeting")
                .route("/hey", web::get().to(handler::manual_hello))
                .service(handler::hello),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}