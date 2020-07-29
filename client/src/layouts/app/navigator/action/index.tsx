import { h } from 'preact'

import { Link } from 'wouter/preact'

import ActionComponent from './types'

import './action.styl'

const Action: ActionComponent = ({ children, href }) => (
    <Link href={href}>
        <a class="action">
            <p class="name">
                {children}
            </p>
            <div class="line" />
        </a>
    </Link>
)

export default Action