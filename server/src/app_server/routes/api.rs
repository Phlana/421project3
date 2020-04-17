// use actix_web::{web, HttpRequest, HttpResponse};
// use serde::{Serialize, Deserialize};
// use crate::app_server::models::game::{Game};
// use r2d2::Pool;
// use r2d2_sqlite::SqliteConnectionManager;
// pub use crate::app_server::models::db_object::{DBObject, Documentable}; 

// #[derive(Debug, Serialize, Deserialize)]
// pub struct PostBody {
//     game_number: String,
//     player_1_name: String,
//     player_2_name: String,
//     winner_name: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct PostIdBody {
//     created_by: String,
//     text: String,
// }

// pub async fn get_posts(db: web::Data<Pool<SqliteConnectionManager>>) -> HttpResponse {
//     match Game::find_all(db.get().unwrap()){
//         Ok(data) =>  HttpResponse::Ok().json(data),
//         Err(err) => HttpResponse::InternalServerError().json(err)
//     }
// }

// pub async fn post_posts(body: web::Json<Game>, db: web::Data<Pool<SqliteConnectionManager>>) -> HttpResponse {
//     let mut game = Game::new();
//     game.game_number = Some(body.game_number.clone());
//     game.player_1_name = Some(body.player_1_name.clone());
//     game.player_2_name = Some(body.player_2_name.clone());
//     game.winner_name= Some(body.winner_name.clone());
    
//     println!("Player1Name: {}", body.player_1_name);

//     match game.insert(db.get().unwrap()){
//         Ok(()) =>  HttpResponse::Ok().json(game),
//         Err(err) => HttpResponse::InternalServerError().json(err),
//     }
// }

// pub async fn put_post_id(body: web::Json<Game>, req: HttpRequest, db: web::Data<Pool<SqliteConnectionManager>>) -> HttpResponse {
//     let id = req.match_info().get("id").unwrap();

//     match Game::find_by_id(db.get().unwrap(), id.to_string()) {
//         Ok(mut game) =>  {
//             game.created_by = Some(body.created_by.clone());
//             game.text = Some(body.text.clone());

//             match game.update(db.get().unwrap()) {
//                 Ok(s) =>  HttpResponse::Ok().json(s),
//                 Err(err) => HttpResponse::InternalServerError().json(err),
//             }
//         },
//         Err(err) => HttpResponse::InternalServerError().json(err),
//     }
// }

// pub async fn get_post_id(req: HttpRequest, db: web::Data<Pool<SqliteConnectionManager>>) -> HttpResponse {
//     let id = req.match_info().get("id").unwrap();

//     match Game::find_by_id(db.get().unwrap(), id.to_string()) {
//         Ok(game) =>  HttpResponse::Ok().json(game),
//         Err(err) => HttpResponse::InternalServerError().json(err),
//     }
// }

// pub async fn delete_post_id(req: HttpRequest, db: web::Data<Pool<SqliteConnectionManager>>) -> HttpResponse {
//     let id = req.match_info().get("id").unwrap();

//     match Game::remove_by_id(db.get().unwrap(), id.to_string()) {
//         Ok(game) =>  HttpResponse::Ok().json(game),
//         Err(err) => HttpResponse::InternalServerError().json(err),
//     }
// }