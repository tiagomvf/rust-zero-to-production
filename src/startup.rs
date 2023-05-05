use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use crate::routes::health_check;
use crate::routes::add_subscriber;

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
