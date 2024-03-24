use std::net::TcpListener;

use sqlx::PgPool;
use zero_to_prod_example::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main] // <- this is the same as tokio::main
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber(
        "zero_to_prod_example".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(&address).expect("Failed to bind random port");
    println!("Server running on: http://{}", address);
    run(listener, connection_pool)?.await
}
