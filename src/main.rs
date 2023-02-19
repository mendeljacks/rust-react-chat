#[macro_use]
extern crate diesel;

use actix::*;
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{http, web, App, HttpServer};

use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};
use dotenvy::var;

mod db;
mod models;
mod routes;
mod schema;
mod server;
mod session;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = server::ChatServer::new().start();

    let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let server_addr = "0.0.0.0";
    let server_port = 8080;

    let app = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(server.clone()))
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .service(web::resource("/").to(routes::index))
            .route("/ws", web::get().to(routes::chat_server))
            .service(routes::create_user)
            .service(routes::get_user_by_id)
            .service(routes::get_user_by_username)
            .service(routes::get_message_by_id)
            .service(routes::get_rooms)
            .service(Files::new("/", "./static"))
    })
    .workers(2)
    .bind((server_addr, server_port))?
    .run();

    println!("Server running at http://{server_addr}:{server_port}/");
    app.await
}
