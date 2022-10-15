use zero2prod::{startup::run, configuration::get_configuration};
use std::net::TcpListener;


#[tokio::main]
async fn main() -> std::io::Result<()> {

    // Parse environment
    let environment  = std::env::var("ENVIRONMENT").map_or(None, |v| Some(v));

    let config = dbg!(get_configuration(environment).expect("could not parse config"));
    let listener = TcpListener::bind(format!("0.0.0.0:{}",config.application_port))?;
    run(listener)?.await
}