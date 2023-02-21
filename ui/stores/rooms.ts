import { observable } from 'mobx'

export const rooms = observable({
    is_loading: false,
    rooms: []
})
