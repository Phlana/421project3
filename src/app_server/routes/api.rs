use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use crate::app_server::models::game::{Game};
pub use crate::app_server::models::db_object::DBObject;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostBody {
    game_number: String,
    player_1_name: String,
    player_2_name: String,
    winner_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostIdBody {
    created_by: String,
    text: String,
}

pub async fn get_posts() -> HttpResponse {
    match Game::find_all(){
        Ok(data) =>  HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::InternalServerError().json(err)
    }
}

pub async fn post_posts(body: web::Json<PostBody>) -> HttpResponse {
    let mut game = Game::new();
    game.game_number = body.game_number.clone();
    game.player_1_name = body.player_1_name.clone();
    game.player_2_name = body.player_2_name.clone();
    game.winner_name= body.winner_name.clone();
    
    println!("Player1Name: {}", body.player_1_name);

    match game.save(){
        Ok(()) =>  HttpResponse::Ok().json(game),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

pub async fn put_post_id(body: web::Path<PostIdBody>, req: HttpRequest) -> HttpResponse {
    let id = req.match_info().get("id").unwrap();

    match Game::find_by_id(id.to_string()) {
        Ok(mut game) =>  {
            game.created_by = body.created_by.clone();
            game.text = body.text.clone();

            match game.save() {
                Ok(()) =>  HttpResponse::Ok().json(game),
                Err(err) => HttpResponse::InternalServerError().json(err),
            }
        },
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

pub async fn get_post_id(req: HttpRequest) -> HttpResponse {
    let id = req.match_info().get("id").unwrap();

    match Game::find_by_id(id.to_string()) {
        Ok(game) =>  HttpResponse::Ok().json(game),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

pub async fn delete_post_id(req: HttpRequest) -> HttpResponse {
    let id = req.match_info().get("id").unwrap();

    match Game::remove_by_id(id.to_string()) {
        Ok(game) =>  HttpResponse::Ok().json(game),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}