use crate::constants::{error_handler, DEFAULT_HTTP_USER_AGENT};
use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{middleware, web, App, HttpServer};
use std::{env, io};
use web3::transports::Http;
use web3::Web3;

mod constants;
mod mongo;
mod routes;
mod tools;

#[tokio::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=debug");
    env_logger::init();

    let mut db_connection = mongo::Connection::new("mongodb://127.0.0.1:27017").await;
    let db = db_connection.database("roninchain").clone();

    let ronin = Web3::new(Http::new("http://127.0.0.1:8545").unwrap());

    HttpServer::new(move || {
        let default_headers =
            middleware::DefaultHeaders::new().add(("Server", DEFAULT_HTTP_USER_AGENT));

        App::new()
            .wrap(Cors::permissive())
            .wrap(default_headers)
            .app_data(Data::new(db.clone()))
            .app_data(Data::new(ronin.clone()))
            .wrap(middleware::Logger::default())
            .service(routes::history::wallet_history)
            .default_service(web::route().to(error_handler))
    })
    .bind("127.0.0.1:9090")?
    .run()
    .await
}
