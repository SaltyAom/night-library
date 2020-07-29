import { h, Fragment } from 'preact'

import { Link } from 'wouter/preact'

import { useStoreon } from 'storeon/preact'
import { UserStore, UserEvent } from '../../../stores/types'

import Action from './action'

import './navigator.styl'

const Navigator = () => {
    let { username } = useStoreon<UserStore, UserEvent>('username')

    return (
        <nav id="navigator">
            <section class="header">
                <Link href="/">
                    <a class="title">
                        <h1 class="value">Night Library</h1>
                    </a>
                </Link>
            </section>
            <section id="action-list">
                {username ? (
                    <Fragment>
                        <Action href="/my-book">My Book</Action>
                        <Action href="/me">{username}</Action>
                    </Fragment>
                ) : (
                    <Fragment>
                        <Action href="/signin">Signin</Action>
                        <Action href="/signup">Signup</Action>
                    </Fragment>
                )}
            </section>
        </nav>
    )
}

export default Navigator
