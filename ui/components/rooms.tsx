import { runInAction } from 'mobx'
import { observer } from 'mobx-react-lite'
import * as moment from 'moment'
import { useEffect, useState } from 'react'
import { getRooms } from '../services/rooms'
import { rooms_store } from '../stores/store'
import { Room, RoomResponse } from '../types/types'
import Avatar from './avatar'

function RoomListItem({ onSelect, room, userId, index, selectedItem }) {
    const { room_has_users, name, created_at } = room
    const active = index == selectedItem

    return (
        <div
            onClick={() => onSelect(index, {})}
            className={`${
                active
                    ? 'bg-[#FDF9F0] border border-[#DEAB6C]'
                    : 'bg-[#FAF9FE] border border-[#FAF9FE]'
            } p-2 rounded-[10px] shadow-sm cursor-pointer`}
        >
            <div className='flex justify-between items-center gap-3'>
                <div className='flex gap-3 items-center w-full'>
                    <Avatar>{name}</Avatar>
                    <div className='w-full max-w-[150px]'>
                        <h3 className='font-semibold text-sm text-gray-700'>{name}</h3>
                        <p className='font-light text-xs text-gray-600 truncate'>
                            {room_has_users?.length} Users in room
                        </p>
                    </div>
                </div>
                <div className='text-gray-400 min-w-[60px]'>
                    <span className='text-xs'>{moment.utc(created_at).fromNow()}</span>
                </div>
            </div>
        </div>
    )
}

export const fetch_rooms = async () => {
    rooms_store.is_loading = true
    const data = await getRooms()

    runInAction(() => {
        rooms_store.rooms = data
        rooms_store.is_loading = false
    })
}

export const RoomList = observer(
    ({ onRoomChange, userId }: { onRoomChange: Function; userId: number }) => {
        const [selectedItem, setSelectedItem] = useState(-1)

        useEffect(() => {
            runInAction(() => {
                rooms_store.is_loading = true
            })
            fetch_rooms()
        }, [])

        const onSelectedRoom = (idx, room: RoomResponse) => {
            setSelectedItem(idx)
            let mapUsers = new Map()
            room.room_has_users.forEach(el => {
                mapUsers.set(el.id, el)
            })
            // const users = {
            //     get: id => {
            //         return mapUsers.get(id)?.username
            //     },
            //     get_target_user: id => {
            //         return room.room_has_users
            //             .filter(el => el.id != id)
            //             .map(el => el.user.username)
            //             .join('')
            //     }
            // }
            onRoomChange(room)
        }

        return (
            <div className='overflow-hidden space-y-3 mt-3'>
                {rooms_store.is_loading && <p>Loading room lists.</p>}
                {rooms_store.rooms?.map((item, index) => {
                    return (
                        <RoomListItem
                            onSelect={idx => onSelectedRoom(idx, item)}
                            room={item}
                            index={index}
                            key={index}
                            userId={userId}
                            selectedItem={selectedItem}
                        />
                    )
                })}
            </div>
        )
    }
)
