use actix_web::{ error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use crate::app_server::routes::api;
use crate::app_server::routes::index;
use crate::app_server::routes::users;

#[actix_rt::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(
            web::resource("/games")
                .route(web::get().to(index::get_games))
                .route(web::post().to(index::post_game)))
        .service(
            web::resource("/games/{id}")
                .route(web::put().to(index::put_game_id))
                .route(web::get().to(index::get_game_id))
                .route(web::delete().to(index::delete_game_id)))
        .service(
            web::resource("/api/posts")
                .route(web::get().to(api::get_posts))
                .route(web::post().to(api::post_posts)))
        .service(
            web::resource("/api/posts/{id}")
                .route(web::put().to(api::put_post_id))
                .route(web::get().to(api::get_post_id))
                .route(web::delete().to(api::delete_post_id)))
        .service(
            web::resource("/users")
                .route(web::get().to(users::get_users)))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}