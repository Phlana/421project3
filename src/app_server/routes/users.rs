use actix_web::{web, HttpResponse, HttpRequest };
use crate::app_server::models::user::{User};
pub use crate::app_server::models::db_object::{DBObject, Documentable};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserIdBody {
    pub name: Option<String>,
    pub hashed_password: Option<String>,
}

pub async fn get_users(db: web::Data<Pool<SqliteConnectionManager>>) -> HttpResponse {
    match User::find_all(db.get().unwrap()){
        Ok(data) =>  HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::InternalServerError().json(err)
    }
    
    // HttpResponse::Ok().json(UserIdBody { id: "1".to_string(), name: "ABC".to_string(), hashed_password: "pass".to_string()})
}

pub async fn post_user(body: web::Json<User>, db: web::Data<Pool<SqliteConnectionManager>>) -> HttpResponse {
    // println!("{}", body.id);
    // HttpResponse::InternalServerError().json("Not Implmeneted")
    // HttpResponse::Ok().json(User { id: Some("1".to_string()), name: None, hashed_password: Some("pass".to_string())})
    // HttpResponse::Ok().json(UserIdBody { id: "1".to_string(), name: "ABC".to_string(), hashed_password: "pass".to_string()})
    // HttpResponse::Ok().json(user)
    if body.id == None || body.hashed_password == None {
        return HttpResponse::InternalServerError().json("New users must have both a user id and a hashed password");
    }
    match User::find_by_id(db.get().unwrap(), body.get_id().unwrap()) {
        Ok(_) => HttpResponse::InternalServerError().json("User with specified id already exists"),
        Err(_) => {
            let mut user = User {
                id: body.id.clone(),
                name: body.name.clone(),
                hashed_password: body.hashed_password.clone()
            };
            match user.insert(db.get().unwrap()) {
                Ok(()) =>  HttpResponse::Ok().json(user),
                Err(err) => HttpResponse::InternalServerError().json(err),
            }
        },
    }
}

pub async fn put_user_id(body: web::Json<UserIdBody>, req: HttpRequest, db: web::Data<Pool<SqliteConnectionManager>>) -> HttpResponse {
    let id = req.match_info().get("id").unwrap();

    match User::find_by_id(db.get().unwrap(), id.to_string()) {
        Ok(mut user) =>  {
            user.name = body.name.clone();
            user.hashed_password = body.hashed_password.clone();

            match user.update(db.get().unwrap()) {
                Ok(s) =>  HttpResponse::Ok().json(s),
                Err(err) => HttpResponse::InternalServerError().json(err),
            }
        },
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

pub async fn get_user_id(req: HttpRequest, db: web::Data<Pool<SqliteConnectionManager>>) -> HttpResponse {
    let id = req.match_info().get("id").unwrap();

    match User::find_by_id(db.get().unwrap(), id.to_string()) {
        Ok(user) =>  HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
    // HttpResponse::Ok().json(User { id: Some("1".to_string()), name: None, hashed_password: Some("pass".to_string())})
}

pub async fn delete_user_id(req: HttpRequest, db: web::Data<Pool<SqliteConnectionManager>>) -> HttpResponse {
    let id = req.match_info().get("id").unwrap();

    match User::remove_by_id(db.get().unwrap(), id.to_string()) {
        Ok(user) =>  HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}