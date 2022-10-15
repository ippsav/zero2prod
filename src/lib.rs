use actix_web::{web::get, App, HttpResponse, HttpServer, Responder, dev::Server};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run() -> std::io::Result<Server> {
    let server = HttpServer::new(|| App::new().route("/health_check", get().to(health_check)))
        .bind("0.0.0.0:3000")?
        .run();

    Ok(server)
}
