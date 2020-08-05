import { useEffect } from 'preact/hooks'

import { useStoreon } from 'storeon/preact'
import { UserStore, UserEvent } from '../../stores/types'

import RefreshProviderComponent from './types'

const RefreshProvider: RefreshProviderComponent = ({ children }) => {
    let { dispatch } = useStoreon<UserStore, UserEvent>()

    useEffect(() => {
        fetch(`http://localhost/api/refresh`, {
            method: 'POST',
            credentials: 'include'
        })
        .then(res => res.text())
        .then(username => {
            if(username)
                dispatch("UPDATE_USERNAME", username)
            else
                dispatch("LOGOUT", '')
        })
    }, [])    
    
    return children
}

export default RefreshProvider