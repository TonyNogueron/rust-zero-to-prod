// tokio::test is equivalent to tokio::main, but it's used for tests. and saves us
// from using the #[test] attribute.

use std::net::TcpListener;

use zero_to_prod_example::FormData;

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
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let json_body = zero_to_prod_example::FormData {
        name: "George".to_string(),
        email: "george_t@gmail.com".to_string(),
    };

    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&json_body).expect("Failed to parse the body to a json string"))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_code_when_data_is_missing() {
    // Arrange
    let address = spawn_app();
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
            .post(&format!("{}/subscriptions", &address))
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
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
