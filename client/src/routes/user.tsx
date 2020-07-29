import { h } from 'preact'
import { useReducer } from 'preact/hooks'

import { Link, Redirect } from 'wouter/preact'

import { useStoreon } from 'storeon/preact'
import { UserStore, UserEvent } from '../stores/types'

import '../styles/user.styl'

const User = () => {
    let { dispatch } = useStoreon<UserStore, UserEvent>()

    let [signout, requestSignout] = useReducer(_ => true, false)

    let { username, initial } = useStoreon<UserStore, UserEvent>(
        'username',
        'initial'
    )

    let logout = async () => {
        await fetch('http://localhost/logout', {
            method: 'GET',
            credentials: 'include'
        })

        dispatch('LOGOUT', '')
        requestSignout(1)
    }

    if (signout) return <Redirect href="/" />

    if (!username && !initial) return <Redirect href="/signin" />

    return (
        <section id="user">
            <h1 class="title">Welcome back {username}</h1>
            <aside class="action-list">
                <Link href="/my-book">
                    <a class="action">My Book</a>
                </Link>
                <button class="action" onClick={logout}>
                    Logout
                </button>
            </aside>
        </section>
    )
}

export default User
