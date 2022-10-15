use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::net::TcpListener;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}


#[derive(Deserialize)]
struct FormData {
    pub name: String,
    pub email: String,
}

async fn subscribe(data: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().body(format!("{}:{}",data.name,data.email))
}

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
