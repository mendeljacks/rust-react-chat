import { base_url } from './api'
import axios from 'axios'
import { Message } from '../types/types'

export const fetchRoomData = async room_id => {
    if (!room_id) return
    const url = `${base_url}/messages/${room_id}`
    try {
        let resp = await axios.get(url)
        return resp.data as Message[]
    } catch (e) {
        console.log(e)
    }
}
