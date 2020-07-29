import { useEffect } from 'preact/hooks'

import { useStoreon } from 'storeon/preact'
import { UserStore, UserEvent } from '../../stores/types'

import RefreshProviderComponent from './types'

const RefreshProvider: RefreshProviderComponent = ({ children }) => {
    let { dispatch } = useStoreon<UserStore, UserEvent>()

    useEffect(() => {
        fetch(`http://localhost/api/refresh?time=${Date.now()}`, {
            method: 'POST',
            credentials: 'include'
        })
        .then(res => res.text())
        .then(res => {
            if(res)
                dispatch("UPDATE_USERNAME", res)
            else
                dispatch("LOGOUT", '')
        })
    }, [])    
    
    return children
}

export default RefreshProvider