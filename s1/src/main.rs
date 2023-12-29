use actix_web::{web, App, HttpServer};
use handler::{get_counter, get_state};
use module::{AppState, AppStateWithCounter};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::{io, sync::Mutex};
pub mod handler;
pub mod module;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: "Actix-Web".into(),
            }))
            .app_data(counter.clone())
            .service(
                web::scope("/greeting")
                    .service(handler::hello),
            )
            .service(get_state)
            .route("/counter", web::get().to(get_counter))
    })
    // .workers(num_cpus::get() * 2)
    // .bind("127.0.0.1:8080")?
    .bind_openssl("127.0.0.1:8080", builder)?
    .run()
    .await
}
