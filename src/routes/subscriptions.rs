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

pub async fn subscribe(data: web::Json<FormData>, db_pool: web::Data<PgPool>) -> impl Responder {
    match validate_form_data(data) {
        Ok(d) => {
            match sqlx::query!(
                r#"
                    INSERT INTO subscriptions (id, email, name, subscribed_at)
                    VALUES ($1, $2, $3, $4)
                    "#,
                Uuid::new_v4(),
                d.email,
                d.name,
                Utc::now()
            )
            .execute(db_pool.get_ref())
            .await
            {
                Ok(_) => HttpResponse::Ok().body(format!("Received JSON data: {:?}", d)),
                Err(e) => {
                    println!("Failed to execute query: {}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(e) => HttpResponse::BadRequest().body(format!("Bad Request: {:?}", e)),
    }
}

// pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
//     match sqlx::query!(
//         r#"
//     INSERT INTO subscriptions (id, email, name, subscribed_at)
//     VALUES ($1, $2, $3, $4)
//             "#,
//         Uuid::new_v4(),
//         form.email,
//         form.name,
//         Utc::now()
//     )
//     .execute(pool.as_ref())
//     .await
//     {
//         Ok(_) => HttpResponse::Ok().finish(),
//         Err(e) => {
//             println!("Failed to execute query: {}", e);
//             HttpResponse::InternalServerError().finish()
//         }
//     }
// }
