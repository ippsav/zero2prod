use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct FormData {
    pub name: String,
    pub email: String,
}

#[tracing::instrument(
        name="Adding a new subsriber",
        skip_all,
        fields(
                request_id = %Uuid::new_v4(),
                subscriber_email = %data.email,
                subscriber_name = %data.name
                )
        )]
pub async fn subscribe(data: web::Form<FormData>, pool: web::Data<PgPool>) -> impl Responder {
    tracing::info_span!("Saving new subscriber details");
    match insert_subscriber(&data, pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok(),
        Err(err) => {
            tracing::error!("error is: {err:?}");
            HttpResponse::InternalServerError()
        }
    }
}

#[tracing::instrument(
        name = "Saving new subscriber details"
        skip_all
        )]
async fn insert_subscriber(data: &FormData, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO subscriptions VALUES($1,$2,$3,$4)")
        .bind(Uuid::new_v4())
        .bind(&data.email)
        .bind(&data.name)
        .bind(Utc::now())
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("could not insert subscriber to database: {e:?}");
            e
        })?;
    Ok(())
}
