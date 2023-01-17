use std::{io};
use password_creation::HashData;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;


mod password_creation;
mod password_verification;
mod password_manager_http;

#[actix_web::main]
async fn main() -> io::Result<()> {

    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            // register HTTP requests handlers
            .service(password_manager_http::verify)
            .service(password_manager_http::create)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
