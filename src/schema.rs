// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Int4,
        room_id -> Int4,
        user_id -> Int4,
        content -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    room_has_users (id) {
        id -> Int4,
        room_id -> Int4,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    rooms (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(messages -> rooms (room_id));
diesel::joinable!(messages -> users (user_id));
diesel::joinable!(room_has_users -> rooms (room_id));
diesel::joinable!(room_has_users -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    messages,
    room_has_users,
    rooms,
    users,
);
