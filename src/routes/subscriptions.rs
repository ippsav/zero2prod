use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
}

pub async fn subscribe(data: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().body(format!("{}:{}", data.name, data.email))
}
