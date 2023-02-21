import { base_url } from './api'
import axios from 'axios'
import { User } from '../types/types'

export const createUser = async ({ username }: { username: string }) => {
    try {
        const url = `${base_url}/users/create`
        let result = await axios.post(url, {
            username
        })
        return result.data
    } catch (e) {
        return Promise.reject(e)
    }
}

export const getUser = async ({ username }: { username: string }) => {
    try {
        const url = `${base_url}/users/username/` + username
        let result = await axios.get(url)
        return result.data as User
    } catch (e) {
        console.log(e)
        return Promise.reject(e)
    }
}
