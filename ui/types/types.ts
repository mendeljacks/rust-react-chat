export type User = {
    id: number
    username: string
    created_at: string
    updated_at: string
}

export type NewUser = {
    username: string
    created_at?: string
    updated_at?: string
}

export type Room = {
    id: number
    name: string
    created_at: string
    updated_at: string
}

export type NewRoom = {
    name: string
    created_at?: string
    updated_at?: string
}

export type Message = {
    id: number
    room_id: number
    user_id: number
    content: string
    created_at: string
    updated_at: string
}

export type NewMessage = {
    room_id: number
    user_id: number
    content: string
    created_at?: string
    updated_at?: string
}

export type RoomHasUser = {
    id: number
    room_id: number
    user_id: number
    created_at: string
    updated_at: string
}

export type NewRoomHasUser = {
    room_id: number
    user_id: number
    created_at?: string
    updated_at?: string
}

export interface RoomHasUserWithUser extends RoomHasUser {
    user: User
}

export type RoomResponse = {
    id: number
    name: string
    created_at: string
    updated_at: string
    room_has_users: RoomHasUserWithUser[]
}

export interface MessageResponse extends Omit<Message, 'user_id'> {
    message: string
    user: User
}
