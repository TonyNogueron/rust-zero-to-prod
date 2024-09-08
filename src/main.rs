use std::net::TcpListener;
use zero_to_prod_example::email_client::EmailClient;

use sqlx::postgres::PgPoolOptions;
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
    let connection_pool = PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());

    // Build an `EmailClient` using `configuration`
    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address.");
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
    );

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let listener = TcpListener::bind(&address).expect("Failed to bind random port");
    println!("Server running on: http://{}", address);
    run(listener, connection_pool, email_client)?.await;
    Ok(())
}
