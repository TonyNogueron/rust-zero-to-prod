// tokio::test is equivalent to tokio::main, but it's used for tests. and saves us
// from using the #[test] attribute.

#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = zero_to_prod_example::run().expect("Failed to bind address");
    // We launch the server in a background task
    // tokio::spawn returns a handle to the spawned future, but we don't need it here
    let _ = tokio::spawn(server);
}

