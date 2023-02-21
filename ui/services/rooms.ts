import { base_url } from './api'
import axios from 'axios'
import { Room } from '../types/types'

export const getRooms = async () => {
    try {
        const url = `${base_url}/rooms`
        let result = await axios.get(url)

        return result.data as Room[]
    } catch (e) {
        return Promise.reject(e)
    }
}

export const ensureRoomExists = async (name: string) => {
    // If user is in room do nothing
    // If user is not in room add user to room
    // If room does not exist create room and add user to room
    try {
        // get room by name
        const url = `${base_url}/rooms/${name}`
        let result = await axios.get(url)

        // if room does not exist create room
        if (result.data === null) {
            result = await createRoom(name)
        }
    } catch (e) {
        return Promise.reject(e)
    }
}

export const createRoom = async (name: string) => {
    try {
        const url = `${base_url}/rooms/create`
        let result = await axios.post(url, {
            name
        })

        return result.data
    } catch (e) {
        return Promise.reject(e)
    }
}
