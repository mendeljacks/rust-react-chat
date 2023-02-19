import { useEffect, useState } from 'react'
import { base_url } from '../components/rooms'

const fetchRoomData = async room_id => {
    if (!room_id) return
    const url = `${base_url}/messages/${room_id}`
    try {
        let resp = await fetch(url).then(res => res.json())
        return resp
    } catch (e) {
        console.log(e)
    }
}

export default function useMessages(room_id) {
    const [isLoading, setIsLoading] = useState(true)
    const [messages, setMessages] = useState([])

    const updateMessages = (resp = []) => {
        setIsLoading(false)
        setMessages(resp)
    }

    const fetchMessages = id => {
        setIsLoading(true)
        fetchRoomData(id).then(updateMessages)
    }

    useEffect(() => fetchMessages(room_id), [])

    return [isLoading, messages, setMessages, fetchMessages]
}
