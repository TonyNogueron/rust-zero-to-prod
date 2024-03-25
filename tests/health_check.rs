// tokio::test is equivalent to tokio::main, but it's used for tests. and saves us
// from using the #[test] attribute.

use std::net::TcpListener;

use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use zero_to_prod_example::{
    configuration::{get_configuration, DatabaseSettings},
    routes::FormData,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    // We cannot assign the output of `get_subscriber` to a variable based on the
    // value TEST_LOG` because the sink is part of the type returned by
    // `get_subscriber`, therefore they are not the same type. We could work around
    // it, but this is the most straight-forward way of moving forward.
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create new database with a the random name to insolate the test
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}

async fn spawn_app() -> TestApp {
    // The first time `initialize` is invoked the code in `TRACING` is executed. // All other invocations will instead skip execution.
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;

    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // We launch the server in a background task
    // tokio::spawn returns a handle to the spawned future, but we don't need it here
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let json_body = FormData {
        name: "George".to_string(),
        email: "george_t@gmail.com".to_string(),
    };

    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&json_body).expect("Failed to parse the body to a json string"))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscriptions.");

    assert_eq!(json_body.email, saved.email);
    assert_eq!(json_body.name, saved.name);
}

#[tokio::test]
async fn subscribe_returns_a_400_code_when_data_is_invalid() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases: Vec<(FormData, String)> = vec![
        (
            FormData {
                name: "Le guin".to_string(),
                email: "".to_string(),
            },
            "missing the email".to_string(),
        ),
        (
            FormData {
                name: "".to_string(),
                email: "ursula_le_guin@gmail.com".to_string(),
            },
            "missing the name".to_string(),
        ),
        (
            FormData {
                name: "".to_string(),
                email: "".to_string(),
            },
            "missing both name and email".to_string(),
        ),
        (
            FormData {
                name: "Tony".to_string(),
                email: "not_valid_email".to_string(),
            },
            "has an invalid email".to_string(),
        ),
    ];

    // Act
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/json")
            .body(
                serde_json::to_string(&invalid_body)
                    .expect("Failed to parse the invalid body to a json string"),
            )
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            // Not 200 anymore!
            400,
            response.status().as_u16(),
            "The API did not return a 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
