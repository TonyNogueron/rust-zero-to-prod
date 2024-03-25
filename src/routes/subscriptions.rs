use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};

#[derive(Debug, Deserialize, Serialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
}

impl TryFrom<web::Json<FormData>> for NewSubscriber {
    type Error = String;
    fn try_from(value: web::Json<FormData>) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name.clone())?;
        let email = SubscriberEmail::parse(value.email.clone())?;
        Ok(Self { email, name })
    }
}

#[tracing::instrument(
name = "Adding a new subscriber", skip(data, db_pool),
fields(
subscriber_email = %data.email, subscriber_name = %data.name
) )]
pub async fn subscribe(data: web::Json<FormData>, db_pool: web::Data<PgPool>) -> impl Responder {
    let new_subscriber = match data.try_into() {
        Ok(subscriber) => subscriber,
        Err(_) => return HttpResponse::BadRequest().body("Invalid data."),
    };

    match insert_subscriber(&new_subscriber, &db_pool).await {
        Ok(_) => HttpResponse::Ok().body(format!("Received JSON data: {:?}", new_subscriber)),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, db_pool)
)]
pub async fn insert_subscriber(
    new_subscriber: &NewSubscriber,
    db_pool: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
"#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
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
