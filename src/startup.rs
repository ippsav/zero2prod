use tracing_actix_web::TracingLogger;
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;

use std::net::TcpListener;

use crate::routes::{health_check, subscribe};

pub fn run(listener: TcpListener, db_pool: PgPool) -> std::io::Result<Server> {
    let app_data = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(app_data.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
