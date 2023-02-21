export const hostname =
    typeof window !== 'undefined' && window.location.hostname !== 'localhost'
        ? 'mendelchat.fly.dev'
        : 'localhost:8080'

export const base_url =
    typeof window !== 'undefined' && window.location.hostname === 'localhost'
        ? `http://${hostname}`
        : `https://${hostname}`
