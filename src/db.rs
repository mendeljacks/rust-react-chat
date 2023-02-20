use chrono::Utc;
use diesel::prelude::*;

use crate::{
    models::{
        Message, NewMessage, NewUser, Room, RoomHasUser, RoomHasUserWithUser, RoomResponse, User,
    },
    schema,
};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_user_by_id(conn: &mut PgConnection, id: i32) -> Result<Option<User>, DbError> {
    let user = schema::users::table
        .filter(schema::users::id.eq(id))
        .first::<User>(conn)
        .optional()?;

    Ok(user)
}

pub fn get_messages_by_room_id(
    conn: &mut PgConnection,
    id: i32,
) -> Result<Option<Vec<Message>>, DbError> {
    use crate::schema::messages;

    let convo = messages::table
        .filter(messages::room_id.eq(id))
        .load(conn)
        .optional()?;

    Ok(convo)
}

pub fn find_user_by_username(
    conn: &mut PgConnection,
    username: String,
) -> Result<Option<User>, DbError> {
    let user = schema::users::table
        .filter(schema::users::username.eq(username))
        .first::<User>(conn)
        .optional()?;
    Ok(user)
}

pub fn get_all_rooms(conn: &mut PgConnection) -> Result<Vec<RoomResponse>, DbError> {
    let rooms_data: Vec<Room> = schema::rooms::table.get_results(conn)?;
    let room_ids = rooms_data.iter().map(|room| room.id).collect::<Vec<_>>();

    let room_has_users_data: Vec<RoomHasUser> = schema::room_has_users::table
        .filter(schema::room_has_users::room_id.eq_any(room_ids))
        .get_results(conn)?;
    let room_has_user_ids = room_has_users_data
        .iter()
        .map(|room_has_user| room_has_user.user_id)
        .collect::<Vec<_>>();

    let users_data: Vec<User> = schema::users::table
        .filter(schema::users::id.eq_any(room_has_user_ids))
        .get_results(conn)?;

    // Put users into the room has users data
    let room_has_users: Vec<RoomHasUserWithUser> = room_has_users_data
        .iter()
        .map(|room_has_user| {
            let user = users_data
                .iter()
                .find(|user| user.id == room_has_user.user_id)
                .unwrap();

            RoomHasUserWithUser {
                id: room_has_user.id,
                room_id: room_has_user.room_id,
                user_id: room_has_user.user_id,
                created_at: room_has_user.created_at,
                updated_at: room_has_user.updated_at,
                user: user.clone(),
            }
        })
        .collect();

    // Put room has users into the rooms data
    let rooms: Vec<RoomResponse> = rooms_data
        .iter()
        .map(|room| {
            let room_has_users = room_has_users
                .iter()
                .filter(|room_has_user| room_has_user.room_id == room.id)
                .map(|room_has_user| room_has_user.clone())
                .collect::<Vec<_>>();

            RoomResponse {
                id: room.id,
                name: room.name.clone(),
                created_at: room.created_at,
                updated_at: room.updated_at,
                room_has_users,
            }
        })
        .collect();

    Ok(rooms)
}

pub fn insert_new_user(conn: &mut PgConnection, nm: &str) -> Result<NewUser, DbError> {
    use crate::schema::users::dsl::*;

    let new_user = NewUser {
        username: nm.to_owned(),
        created_at: Some(Utc::now().naive_utc()),
        updated_at: Some(Utc::now().naive_utc()),
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(new_user)
}

pub fn insert_new_message(conn: &mut PgConnection, new: NewMessage) -> Result<NewMessage, DbError> {
    use crate::schema::messages::dsl::*;

    let new_message = NewMessage {
        user_id: new.user_id,
        room_id: new.room_id,
        content: new.content,
        created_at: None,
        updated_at: None,
    };

    diesel::insert_into(messages)
        .values(&new_message)
        .execute(conn)?;

    Ok(new_message)
}
