use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Room {
    pub id: i32,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewRoom {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Message {
    pub id: i32,
    pub room_id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMessage {
    pub room_id: i32,
    pub user_id: i32,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct RoomHasUser {
    pub id: i32,
    pub room_id: i32,
    pub user_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewRoomHasUser {
    pub room_id: i32,
    pub user_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomResponse {
    pub id: i32,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub participants: Vec<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    pub id: i32,
    pub room_id: i32,
    pub user_id: i32,
    pub message: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub user: User,
}
