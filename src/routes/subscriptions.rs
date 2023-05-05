use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn add_subscriber(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
