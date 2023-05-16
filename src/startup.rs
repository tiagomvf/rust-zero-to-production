use std::net::TcpListener;

use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
// use env_logger::Env;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;
use web::Data;

pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    let db_poll = Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_poll.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
