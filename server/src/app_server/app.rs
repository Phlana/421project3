use actix_web::{middleware, web, App, HttpServer};
// use crate::app_server::routes::api;
use crate::app_server::routes::index;
use crate::app_server::routes::users;
use std::io;
use r2d2_sqlite::{SqliteConnectionManager};
use rusqlite::{NO_PARAMS, Connection, Error};

pub fn initialize() -> Result<(),Error>{
    let conn = Connection::open("public.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS games (
            id text primary key,
            game_type text,
            player_1_name text,
            player_2_name text,
            winner_name text,
            text text,
            created_by text,
            game_date date
            )",
        NO_PARAMS,
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id text primary key,
            name text,
            hashed_password text not null
            )",
        NO_PARAMS,
    )?;
    Ok(())
}

#[actix_rt::main]
pub async fn run() -> io::Result<()> {
    let manager = SqliteConnectionManager::file("public.db");
    let pool = r2d2::Pool::new(manager).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone()) // <- store db pool in app state
            .wrap(middleware::Logger::default())
            .service(web::resource("/games")
                    .route(web::get().to(index::get_games))
                    .route(web::post().to(index::post_game)))
            .service(web::resource("/games/{id}")
                    .route(web::put().to(index::put_game_id))
                    .route(web::get().to(index::get_game_id))
                    .route(web::delete().to(index::delete_game_id)))
            .service(web::resource("/users")
                    .route(web::get().to(users::get_users))
                    .route(web::post().to(users::post_user)))
            .service(web::resource("/users/{id}")
                    .route(web::put().to(users::put_user_id))
                    .route(web::get().to(users::get_user_id))
                    .route(web::delete().to(users::delete_user_id)))
            .service(web::resource("/api/posts")
                    .route(web::get().to(index::get_games))
                    .route(web::post().to(index::post_game)))
            .service(web::resource("/api/posts/{id}")
                    .route(web::put().to(index::put_game_id))
                    .route(web::get().to(index::get_game_id))
                    .route(web::delete().to(index::delete_game_id)))
            // .service(web::resource("/api/posts")
            //         .route(web::get().to(api::get_posts))
            //         .route(web::post().to(api::post_posts)))
            // .service(web::resource("/api/posts/{id}")
            //         .route(web::put().to(api::put_post_id))
            //         .route(web::get().to(api::get_post_id))
            //         .route(web::delete().to(api::delete_post_id)))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await

    // match HttpServer::new(|| {
    //     App::new()
    //     // .data(pool.clone())
    //     .service(
    //         web::resource("/games")
    //             .route(web::get().to(index::get_games))
    //             .route(web::post().to(index::post_game)))
    //     .service(
    //         web::resource("/games/{id}")
    //             .route(web::put().to(index::put_game_id))
    //             .route(web::get().to(index::get_game_id))
    //             .route(web::delete().to(index::delete_game_id)))
    //     .service(
    //         web::resource("/api/posts")
    //             .route(web::get().to(api::get_posts))
    //             .route(web::post().to(api::post_posts)))
    //     .service(
    //         web::resource("/api/posts/{id}")
    //             .route(web::put().to(api::put_post_id))
    //             .route(web::get().to(api::get_post_id))
    //             .route(web::delete().to(api::delete_post_id)))
    //     .service(
    //         web::resource("/users")
    //             .route(web::get().to(users::get_users)))
    // })
    // .bind("127.0.0.1:3000") {
    //     Ok(b) => { 
    //         match b.run().await{
    //             Ok(_) => Ok(()),
    //             Err(_) => Err(())
    //         }
    //     }
    //     Err(_) => {return Err(());}
    // }
}