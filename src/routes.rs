use std::time::Instant;

use actix::*;
use actix_files::NamedFile;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;

use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};

use crate::db;
use crate::models;
use crate::server;
use crate::session;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html")
        .await
        .expect("Index not found did you build static folder?")
}

pub async fn chat_server(
    req: HttpRequest,
    stream: web::Payload,
    pool: web::Data<DbPool>,
    srv: web::Data<Addr<server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        session::WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "main".to_string(),
            name: None,
            addr: srv.get_ref().clone(),
            db_pool: pool,
        },
        &req,
        stream,
    )
}

#[post("/users/create")]
pub async fn create_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::insert_new_user(&mut conn, &form.username)
    })
    .await?
    .map_err(actix_web::error::ErrorUnprocessableEntity)?;

    Ok(HttpResponse::Ok().json(user))
}

#[post("/rooms/create")]
pub async fn create_room(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewRoom>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::insert_new_room(&mut conn, &form.name)
    })
    .await?
    .map_err(actix_web::error::ErrorUnprocessableEntity)?;

    Ok(HttpResponse::Ok().json(user))
}

#[post("/room_has_users/create")]
pub async fn create_room_has_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewRoomHasUser>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::insert_new_room_has_user(&mut conn, form.room_id, form.user_id)
    })
    .await?
    .map_err(actix_web::error::ErrorUnprocessableEntity)?;

    Ok(HttpResponse::Ok().json(user))
}

#[get("/users/{user_id}")]
pub async fn get_user_by_id(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let user_id = id.to_owned();
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::find_user_by_id(&mut conn, user_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[get("/messages/{uid}")]
pub async fn get_message_by_id(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let room_id = id.to_owned();
    let messages = web::block(move || {
        let mut conn = pool.get()?;
        db::get_messages_by_room_id(&mut conn, room_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(messages))
}

#[get("/users/username/{username}")]
pub async fn get_user_by_username(
    pool: web::Data<DbPool>,
    uname: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let username = uname.to_string();
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::find_user_by_username(&mut conn, username)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[get("/rooms")]
pub async fn get_rooms(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let rooms = web::block(move || {
        let mut conn = pool.get()?;
        db::get_all_rooms(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(rooms))
}

#[get("/rooms/{name}")]
pub async fn get_room_by_name(
    pool: web::Data<DbPool>,
    name: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let room = web::block(move || {
        let mut conn = pool.get()?;
        db::find_room_by_name(&mut conn, name.to_string())
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(room))
}
