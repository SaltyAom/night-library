import { h } from 'preact'
import {
    useState,
    useRef,
    useReducer,
    useCallback,
    useEffect
} from 'preact/hooks'

import { Link, Redirect } from 'wouter/preact'

import '../styles/sign.styl'

const Signin = () => {
    let [error, updateError] = useState(''),
        [redirect, allowRedirect] = useReducer(_ => true, false)

    let username = useRef(''),
        password = useRef(''),
        confirmPassword = useRef('')

    useEffect(() => {
        fetch('http://localhost/api/refresh', {
            method: 'POST',
            credentials: 'include'
        })
            .then(res => res.text())
            .then(res => {
                if (res) allowRedirect(1)
            })
    }, [])

    let updateUsername = useCallback((event: any) => {
            username.current = event.currentTarget.value
        }, []),
        updatePassword = useCallback((event: any) => {
            password.current = event.currentTarget.value
        }, []),
        updateConfirmPassword = useCallback((event: any) => {
            confirmPassword.current = event.currentTarget.value
        }, [])

    let login = useCallback(async (event: Event) => {
        event.preventDefault()

        if (!username.current || !password.current)
            return updateError("Username and Password can't be blank")

        if (password.current !== confirmPassword.current)
            return updateError("Password doesn't match")

        let response = await fetch('http://localhost/register', {
            method: 'POST',
            headers: {
                'Content-Type':
                    'application/x-www-form-urlencoded;charset=UTF-8'
            },
            credentials: 'include',
            body: new URLSearchParams({
                username: username.current,
                password: password.current
            })
        }).then(data => data.json())

        if (response.success || response.info === 'Already signin in')
            allowRedirect(1)
        else updateError(response.info)
    }, [])

    return (
        <form
            id="login"
            action="http://localhost/register"
            method="POST"
            onSubmit={login}
        >
            <h1 class="title">Signin</h1>
            <label class="label" for="username">
                Username
            </label>
            <input
                class="input username"
                name="username"
                type="text"
                placeholder="Username"
                onInput={updateUsername}
            />
            <label class="label" for="password">
                Password
            </label>
            <input
                class="input password"
                name="password"
                type="password"
                placeholder="Password"
                onChange={updatePassword}
            />
            <label class="label" for="confirm-password">
                Confirm Password
            </label>
            <input
                class="input confirm-password"
                name="confirm-password"
                type="password"
                placeholder="Confirm Password"
                onChange={updateConfirmPassword}
            />
            {error && <p class="warning -hidden">{error}</p>}
            <section class="action">
                <button class="login">Signup</button>
                <Link href="/signin">
                    <a class="link">Signin</a>
                </Link>
            </section>
            {redirect && <Redirect href="/" />}
        </form>
    )
}

export default Signin
