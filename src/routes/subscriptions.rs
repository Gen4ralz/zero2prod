use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    // improve log by use correlate all logs to the same request
    let request_id = Uuid::new_v4();

    // improve log detail by log their email and name
    log::info!(
        "request_id {} - Adding '{}' '{}' as a new subscriber",
        request_id,
        form.email,
        form.name
    );

    log::info!(
        "request_id {} - Saving new subscriber in the database",
        request_id
    );

    // 'Result' has two variants: 'OK' and 'Err'
    // Use match statement to choose what to do based
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            log::info!("request_id {} - New subscriber has been saved!", request_id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            // use log::error! and {:?} in debug format
            // will give more detail of error than use println!
            log::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
