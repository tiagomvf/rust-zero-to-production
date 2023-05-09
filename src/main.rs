use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;

use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let db_poll = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed do bind address");
    run(listener, db_poll)?.await
}
