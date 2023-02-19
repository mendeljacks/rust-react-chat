CREATE TABLE room_has_users (
    id integer primary key generated by default as identity,
    room_id int NOT NULL,
    user_id int NOT NULL,
    created_at timestamp NOT NULL DEFAULT NOW(),
    updated_at timestamp NOT NULL DEFAULT NOW(),
    CONSTRAINT room_has_users_room_id_user_id_uq UNIQUE (room_id, user_id),
    CONSTRAINT room_has_users_room_id_fk FOREIGN KEY (room_id) REFERENCES public.rooms(id),
    CONSTRAINT room_has_users_user_id_fk FOREIGN KEY (user_id) REFERENCES public.users(id)
);