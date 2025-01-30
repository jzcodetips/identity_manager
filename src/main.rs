#[macro_use]
extern crate actix_web;

use std::{env, io};
use actix_web::{middleware, App, HttpServer};
use actix_web::web;

mod server;
mod domain;
mod user_store;

use crate::domain::Service;
use crate::user_store::InMemStore;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");

    let user_store = InMemStore::new();
    let service = Service::new(user_store);

    HttpServer::new(move || {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(service.clone()))
            // register HTTP requests handlers
            .service(server::register)
            .service(server::login)
    })
    .bind("0.0.0.0:8888")?
    .run()
    .await
}
