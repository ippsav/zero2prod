use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
}

pub async fn subscribe(data: web::Form<FormData>, pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query("INSERT INTO subscriptions VALUES($1,$2,$3,$4)")
        .bind(Uuid::new_v4())
        .bind(&data.email)
        .bind(&data.name)
        .bind(Utc::now())
        .execute(pool.get_ref())
        .await {
            Ok(_) => HttpResponse::Ok(),
            Err(err) => {
                eprintln!("error is: {err}");
                HttpResponse::InternalServerError()
            },
        }
}
