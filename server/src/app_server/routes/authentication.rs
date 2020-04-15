use actix_web::{ HttpResponse };
// use crate::app_server::models::user::{User};

pub async fn login() -> HttpResponse {
    HttpResponse::InternalServerError().json("Not currently implemented")
}