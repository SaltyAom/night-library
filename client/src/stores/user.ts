import { StoreonModule } from 'storeon'

import { UserStore, UserEvent } from './types'

const user: StoreonModule<UserStore, UserEvent> = store => {
    store.on('@init', () => ({ username: null, initial: true }))

    store.on('UPDATE_USERNAME', (store, username) => ({
        username,
        initial: false
    }))

    store.on('LOGOUT', store => ({
        username: null,
        initial: false
    }))
}

export default user
