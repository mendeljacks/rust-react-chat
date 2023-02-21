import { observable, toJS } from 'mobx'
import { Room, RoomResponse } from '../types/types'

export const rooms_store = observable({
    selected_room: null as RoomResponse | null,
    is_loading: false,
    rooms: []
})

if (typeof window !== 'undefined') {
    // @ts-ignore
    window.rooms_store = rooms_store
    // @ts-ignore
    window.tj = toJS
}
