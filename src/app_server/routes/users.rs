use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use crate::app_server::models::user::{User};
pub use crate::app_server::models::DBObject::DBObject;

pub async fn get_users() -> HttpResponse {
    match User::find_all(){
        Ok(data) =>  HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::InternalServerError().json(err)
    }
}