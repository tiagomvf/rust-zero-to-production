use std::net::TcpListener;

use crate::routes::add_subscriber;
use crate::routes::health_check;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(add_subscriber))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
