use std::net::TcpListener;

use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool, Row};
use uuid::Uuid;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use zero2prod::{
    configuration::{get_configuration, DatabaseSettings},
    startup::run,
};

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber_name = "test";
    let env_filter = "info";

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, env_filter, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, env_filter, std::io::sink);
        init_subscriber(subscriber);
    };
});

struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Connect to database
    let mut conn = PgConnection::connect_with(&config.without_db())
        .await
        .expect("could not connect to database");
    // Create database
    conn.execute(format!(r#"CREATE DATABASE "{}""#, config.db_name).as_str())
        .await
        .expect("could not create database");
    // Closing connection
    conn.close().await.expect("could not close connection");

    // Connect to the created the database returning the pg_pool

    let db_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("could not connect to database");

    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("could not migrate database");

    db_pool
}

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);
    // parsing config
    let config_path = std::env::current_dir().unwrap().join("config");
    let mut config = get_configuration(Some("test".into()), config_path).expect("could not parse configuration");
    config.database.db_name = Uuid::new_v4().to_string();
    // Configuring database
    let db_pool = configure_database(&config.database).await;
    // setting up the listener
    let listener = TcpListener::bind(&config.application.get_address()).unwrap();

    // getting the port from the listener
    config.application.port = listener.local_addr().unwrap().port();
    // setting up the server to be spawned in a tokio task
    let server = run(listener, db_pool.clone()).expect("could not start server");
    let _ = tokio::spawn(server);

    TestApp {
        address: config.application.get_address(),
        db_pool,
    }
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("http://{}/health_check", app.address))
        .send()
        .await
        .expect("could not send request to server");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    //Act
    let body = "name=le%20guin&email=le_guin%40gmail.com";
    let response = client
        .post(format!("http://{}/subscriptions", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("could not send request to server");

    let subscriber = sqlx::query(r#"SELECT email, name FROM subscriptions where name='le guin'"#)
        .fetch_one(&app.db_pool)
        .await
        .expect("could not query subscriber from database");
    assert_eq!(subscriber.get::<String, &str>("name"), "le guin");
    assert_eq!(subscriber.get::<String, &str>("email"), "le_guin@gmail.com");
    assert!(response.status().is_success());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arange
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    // Act

    let test_cases = vec![
        ("name=le%20guin", "missing email"),
        ("email=ursula_le_guin%40gmail.com", "missing name"),
        ("", "missing both"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("http://{}/subscriptions", app.address))
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
