import { action } from 'mobx'
import { observer } from 'mobx-react-lite'
import Head from 'next/head'
import { useEffect, useState } from 'react'
import Avatar from '../components/avatar'
import Login from '../components/login'
import Message from '../components/message'
import { fetch_rooms, RoomList } from '../components/rooms'
import useLocalStorage from '../libs/useLocalStorage'
import useWebsocket from '../libs/useWebsocket'
import { fetchRoomData } from '../services/messages'
import { ensureRoomExists } from '../services/rooms'
import { ensureRoomHasUserExists } from '../services/room_has_user'
import { rooms_store } from '../stores/store'

const Home = observer(() => {
    const room = rooms_store.selected_room
    useEffect(() => fetchMessages(room?.id), [])
    const [isTyping, setIsTyping] = useState(false)
    const [showLogIn, setShowLogIn] = useState(false)
    const [auth, setAuthUser] = useLocalStorage('user', false)

    const [isLoading, setIsLoading] = useState(true)
    const [messages, setMessages] = useState([])

    const updateMessagess = (resp = []) => {
        setIsLoading(false)
        setMessages(resp)
    }

    const fetchMessages = id => {
        setIsLoading(true)
        fetchRoomData(id).then(updateMessagess)
    }

    const handleTyping = mode => {
        if (mode === 'IN') {
            setIsTyping(true)
        } else {
            setIsTyping(false)
        }
    }

    const handleMessage = (msg, userId) => {
        setMessages(prev => {
            const item = { content: msg, user_id: userId }
            return [...prev, item]
        })
    }

    const onMessage = data => {
        try {
            let messageData = JSON.parse(data)
            switch (messageData.chat_type) {
                case 'TYPING': {
                    handleTyping(messageData.value[0])
                    return
                }
                case 'TEXT': {
                    handleMessage(messageData.value[0], messageData.user_id)
                    return
                }
            }
        } catch (e) {
            console.log(e)
        }
    }

    const sendMessage = useWebsocket(onMessage)
    const updateFocus = () => {
        const data = {
            id: 0,
            chat_type: 'TYPING',
            value: ['IN'],
            room_id: room.id,
            user_id: auth.id
        }
        sendMessage(JSON.stringify(data))
    }

    const onFocusChange = () => {
        const data = {
            id: 0,
            chat_type: 'TYPING',
            value: ['OUT'],
            room_id: room.id,
            user_id: auth.id
        }
        sendMessage(JSON.stringify(data))
    }

    const submitMessage = e => {
        e.preventDefault()
        let message = e.target.message.value
        if (message === '') {
            return
        }

        if (!room.id) {
            alert('Please select room!')
            return
        }

        const data = {
            id: 0,
            chat_type: 'TEXT',
            value: [message],
            room_id: room.id,
            user_id: auth.id
        }
        sendMessage(JSON.stringify(data))
        e.target.message.value = ''
        handleMessage(message, auth.id)
        onFocusChange()
    }

    const updateMessages = action(data => {
        if (!data.id) return
        fetchMessages(data.id)
        rooms_store.selected_room = data
    })

    const signOut = () => {
        window.localStorage.removeItem('user')
        setAuthUser(false)
    }

    useEffect(() => setShowLogIn(!auth), [auth])

    const [newRoom, setNewRoom] = useState('')

    return (
        <div>
            <Head>
                <title>Rust with react chat app</title>
                <meta name='description' content='Rust with react chat app' />
                <link rel='icon' href='/favicon.ico' />
            </Head>
            <Login show={showLogIn} setAuth={setAuthUser} />
            <div
                className={`${
                    !auth && 'hidden'
                } bg-gradient-to-b from-orange-400 to-rose-400 h-screen p-12`}
            >
                <main className='flex w-full max-w-[1020px] h-[700px] mx-auto bg-[#FAF9FE] rounded-[25px] backdrop-opacity-30 opacity-95'>
                    <aside className='bg-[#F0EEF5] w-[325px] h-[700px] rounded-l-[25px] p-4 overflow-auto relative'>
                        <span
                            style={{
                                display: 'grid',
                                gridTemplateColumns: '1fr 1fr',
                                placeItems: 'center'
                            }}
                        >
                            <input
                                type='text'
                                value={newRoom}
                                onChange={e => setNewRoom(e.target.value)}
                                className='w-full px-4 py-2 border rounded-[10px] focus:outline-none focus:ring-1 focus:ring-blue-600'
                            />
                            <button
                                disabled={newRoom.length === 0}
                                onClick={async () => {
                                    if (newRoom.length > 0) {
                                        await ensureRoomExists(newRoom)
                                        await ensureRoomHasUserExists(newRoom, auth.id)
                                        await fetch_rooms()
                                    }
                                }}
                                className='text-xs p-3 rounded-[10px] bg-violet-200 font-semibold text-violet-600 text-center'
                            >
                                ADD ROOM
                            </button>
                        </span>
                        <RoomList onRoomChange={updateMessages} userId={auth.id} />
                        <button
                            onClick={signOut}
                            className='text-xs w-full max-w-[295px] p-3 rounded-[10px] bg-violet-200 font-semibold text-violet-600 text-center absolute bottom-4'
                        >
                            LOG OUT
                        </button>
                    </aside>
                    {room?.id && (
                        <section className='rounded-r-[25px] w-full max-w-[690px] grid grid-rows-[80px_minmax(450px,_1fr)_65px]'>
                            <div className='rounded-tr-[25px] w-ful'>
                                <div className='flex gap-3 p-3 items-center'>
                                    <Avatar color='rgb(245 158 11)'>{room.name}</Avatar>
                                    <div>
                                        <p className='font-semibold text-gray-600 text-base'>
                                            {room.name}
                                        </p>
                                        <div className='text-xs text-gray-400'>
                                            {isTyping ? 'Typing...' : '10:15 AM'}
                                        </div>
                                    </div>
                                </div>
                                <hr className='bg-[#F0EEF5]' />
                            </div>
                            {isLoading && room.id && (
                                <p className='px-4 text-slate-500'>Loading messages...</p>
                            )}
                            <Message messages={messages} user={auth} room={room} />
                            <div className='w-full'>
                                <form
                                    onSubmit={submitMessage}
                                    className='flex gap-2 items-center rounded-full border border-violet-500 bg-violet-200 p-1 m-2'
                                >
                                    <input
                                        onBlur={onFocusChange}
                                        onFocus={updateFocus}
                                        name='message'
                                        className='p-2 placeholder-gray-600 text-sm w-full rounded-full bg-violet-200 focus:outline-none'
                                        placeholder='Type your message here...'
                                    />
                                    <button
                                        type='submit'
                                        className='bg-violet-500 rounded-full py-2 px-6 font-semibold text-white text-sm'
                                    >
                                        Sent
                                    </button>
                                </form>
                            </div>
                        </section>
                    )}
                </main>
            </div>
        </div>
    )
})

export default Home
