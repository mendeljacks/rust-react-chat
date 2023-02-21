import axios from 'axios'
import { NewRoom } from '../types/types'
import { base_url } from './api'

export const ensureRoomHasUserExists = async (room_name: string, user_id: number) => {
    // If user is in room do nothing
    // If user is not in room add user to room
    // If room does not exist create room and add user to room
    try {
        // get room by name
        const url = `${base_url}/rooms/${room_name}`
        let result = await axios.get(url)

        // if room does not exist create room
        if (result.data === null) {
            result = await createRoomHasUser(result.data.id, user_id)
        }
    } catch (e) {
        return Promise.reject(e)
    }
}

export const createRoomHasUser = async (room_id: number, user_id: number) => {
    try {
        const url = `${base_url}/room_has_user/create`
        let result = await axios.post(url, {
            room_id,
            user_id
        })

        return result.data
    } catch (e) {
        return Promise.reject(e)
    }
}
