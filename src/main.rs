use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup::run};
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Init Tracing
    let subscriber = get_subscriber("zero2prod", "info",std::io::stdout);
    init_subscriber(subscriber);
    // Parse environment
    let environment = std::env::var("ENVIRONMENT").map_or(None, |v| Some(v));
    //Parse config
    let config = get_configuration(environment).expect("could not parse config");

    // Connect to database
    let db_url = config.database.get_connection_string();
    let db_pool = PgPool::connect(&db_url)
        .await
        .expect("could not connect to database");

    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.application_port))?;
    run(listener, db_pool)?.await
}
