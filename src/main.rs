use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let listener = TcpListener::bind(format!(
        "{}:{}",
        configuration.application_host, configuration.application_port
    ))?;
    run(listener, connection_pool)?.await
}
