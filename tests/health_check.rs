use std::net::TcpListener;

fn spawn_app() -> String {
    // setting up the listener
    let listener = TcpListener::bind("0.0.0.0:0").unwrap();
    // getting the port from the listener
    let port = listener.local_addr().unwrap().port();
    // setting up the server to be spawned in a tokio task
    let server = zero2prod::run(listener).expect("could not start server");
    let _ = tokio::spawn(server);

    format!("0.0.0.0:{port}")
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("http://{address}/health_check"))
        .send()
        .await
        .expect("could not send request to server");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let address = spawn_app();

    let client = reqwest::Client::new();

    //Act
    let body = "name=le%20guin&email=le_guin%40gmail.com";
    let response = client
        .post(format!("http://{address}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("could not send request to server");

    assert!(response.status().is_success());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arange
    let address = spawn_app();

    let client = reqwest::Client::new();

    // Act

    let test_cases = vec![
        ("name=le%20guin", "missing email"),
        ("email=ursula_le_guin%40gmail.com", "missing name"),
        ("", "missing both"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("http://{address}/subscriptions"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("could not send request");
        assert!(
            response.status().is_client_error(),
            "the api didn t fail with 400 Bad Request when the payload was {error_message}"
        );
    }
}
