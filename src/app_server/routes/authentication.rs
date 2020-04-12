use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use crate::app_server::models::user::{User};

pub async fn login() -> HttpResponse {
    HttpResponse::InternalServerError().json("Not currently implemented")
}