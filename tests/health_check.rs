



fn spawn_app() {
    let server = zero2prod::run().expect("could not start server");


    let _ = tokio::spawn(server);
}



#[tokio::test]
async fn health_check_works() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
            .get("http://127.0.0.1:3000/health_check")
            .send()
            .await
            .expect("could not send request to server");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}