export interface UserStore {
    username: string | null
    initial: boolean
}

export interface UserEvent {
    UPDATE_USERNAME: string
    LOGOUT: string
}
