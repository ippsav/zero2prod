use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Init Tracing
    let subscriber = get_subscriber("zero2prod", "info", std::io::stdout);
    init_subscriber(subscriber);
    // Parse environment
    let environment = std::env::var("APP_ENVIRONMENT").map_or(None, |v| Some(v));
    //Parse config
    let config_path = std::env::current_dir()?.join("config");
    let config = get_configuration(environment, config_path).expect("could not parse config");
    let address = config.application.get_address();
    // Connect to database
    let db_config = config.database.with_db();
    let db_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(db_config);

    let listener = TcpListener::bind(address)?;
    run(listener, db_pool)?.await
}
