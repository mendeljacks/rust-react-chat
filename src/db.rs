use chrono::Utc;
use diesel::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::models::{Message, NewMessage, NewUser, Room, RoomResponse, User};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_user_by_id(conn: &mut PgConnection, id: i32) -> Result<Option<User>, DbError> {
    use crate::schema::users::dsl::*;

    let user = users.filter(id.eq(id)).first::<User>(conn).optional()?;

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
    use crate::schema::users::dsl::*;

    let user = users
        .filter(username.eq(username))
        .first::<User>(conn)
        .optional()?;

    Ok(user)
}

pub fn get_all_rooms(conn: &mut PgConnection) -> Result<Vec<RoomResponse>, DbError> {
    use crate::schema::rooms;
    use crate::schema::users;

    let rooms_data: Vec<Room> = rooms::table.get_results(conn)?;
    let mut ids = HashSet::new();
    let mut rooms_map = HashMap::new();
    let data = rooms_data.to_vec();
    for room in &data {
        let user_ids = room
            .participant_ids
            .split(",")
            .into_iter()
            .collect::<Vec<_>>();
        for id in user_ids.to_vec() {
            ids.insert(id.to_string());
        }
        rooms_map.insert(room.id.to_string(), user_ids.to_vec());
    }

    let ids = ids.into_iter().collect::<Vec<_>>();
    let users_data: Vec<User> = users::table
        .filter(users::id.eq_any(ids))
        .get_results(conn)?;
    let users_map: HashMap<String, User> = HashMap::from_iter(
        users_data
            .into_iter()
            .map(|item| (item.id.to_string(), item)),
    );

    let response_rooms = rooms_data
        .into_iter()
        .map(|room| {
            let users = rooms_map
                .get(&room.id.to_string())
                .unwrap()
                .into_iter()
                .map(|id| users_map.get(id.to_owned()).unwrap().clone())
                .collect::<Vec<_>>();
            return RoomResponse { room, users };
        })
        .collect::<Vec<_>>();
    Ok(response_rooms)
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
