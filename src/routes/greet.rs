use actix_web::{Responder, HttpResponse};

pub async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}