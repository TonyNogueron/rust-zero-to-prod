use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::utils::validation::validate_form_data;

#[derive(Debug, Deserialize, Serialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
}

#[tracing::instrument(
name = "Adding a new subscriber", skip(data, db_pool),
fields(
subscriber_email = %data.email, subscriber_name = %data.name
) )]
pub async fn subscribe(data: web::Json<FormData>, db_pool: web::Data<PgPool>) -> impl Responder {
    match validate_form_data(data) {
        Ok(d) => match insert_subscriber(&d, &db_pool).await {
            Ok(_) => HttpResponse::Ok().body(format!("Received JSON data: {:?}", d)),
            Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
        },
        Err(e) => HttpResponse::BadRequest().body(format!("Bad Request: {:?}", e)),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(data, db_pool)
)]
pub async fn insert_subscriber(
    data: &web::Json<FormData>,
    db_pool: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
"#,
        Uuid::new_v4(),
        data.email,
        data.name,
        Utc::now()
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
        // We will talk about error handling in depth later!
    })?;
    Ok(())
}
