// tokio::test is equivalent to tokio::main, but it's used for tests. and saves us
// from using the #[test] attribute.

use std::net::TcpListener;

use actix_web::test;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server = zero_to_prod_example::run(listener).expect("Failed to bind address");
    // We launch the server in a background task
    // tokio::spawn returns a handle to the spawned future, but we don't need it here
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}


#[tokio::test]
async fn subscribe_returns_a_400_code_when_data_is_missing() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    // Act
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status().as_u16(), 400);
}