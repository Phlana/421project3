use actix_web::{ web, HttpRequest, HttpResponse };
use serde::{Serialize, Deserialize};
use crate::app_server::models::game::{Game};
pub use crate::app_server::models::db_object::{DBObject, Documentable}; 
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct GameBody {
//     game_number: String,
//     game_type: String,
//     player_1_name: String,
//     player_2_name: String,
//     winner_name: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct GameIdBody {
//     game_number: String,
//     player_1_name: String,
//     player_2_name: String,
//     winner_name: String,
// }

pub async fn get_games(db: web::Data<Pool<SqliteConnectionManager>>) -> HttpResponse {
    match Game::find_all(db.get().unwrap()){
        Ok(data) =>  HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::InternalServerError().json(err)
    }
}

pub async fn post_game(body: web::Json<Game>, db: web::Data<Pool<SqliteConnectionManager>>) -> HttpResponse {
    let mut game = Game::new();
    game.game_type = body.game_type.clone();
    game.game_number = body.game_number.clone();
    game.player_1_name = body.player_1_name.clone();
    game.player_2_name = body.player_2_name.clone();
    game.winner_name  = body.winner_name.clone();

    match game.insert(db.get().unwrap()){
        Ok(()) =>  HttpResponse::Ok().json(game),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
} 

pub async fn put_game_id(body: web::Json<Game>, req: HttpRequest, db: web::Data<Pool<SqliteConnectionManager>>) -> HttpResponse {
    let id = req.match_info().get("id").unwrap();

    match Game::find_by_id(db.get().unwrap(), id.to_string()) {
        Ok(_) =>  {
            let mut game = Game::new();
            game.game_number = Some(id.to_string());
            game.player_1_name = body.player_1_name.clone();
            game.player_2_name = body.player_2_name.clone();
            game.winner_name = body.winner_name.clone();

            match game.update(db.get().unwrap()) {
                Ok(s) =>  HttpResponse::Ok().json(s),
                Err(err) => HttpResponse::InternalServerError().json(err),
            }
        },
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

pub async fn get_game_id(req: HttpRequest, db: web::Data<Pool<SqliteConnectionManager>>) -> HttpResponse {
    let id = req.match_info().get("id").unwrap();

    match Game::find_by_id(db.get().unwrap(), id.to_string()) {
        Ok(game) =>  HttpResponse::Ok().json(game),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

pub async fn delete_game_id(req: HttpRequest, db: web::Data<Pool<SqliteConnectionManager>>) -> HttpResponse {
    let id = req.match_info().get("id").unwrap();

    match Game::remove_by_id(db.get().unwrap(), id.to_string()) {
        Ok(game) =>  HttpResponse::Ok().json(game),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}