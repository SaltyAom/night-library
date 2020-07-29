import { h } from 'preact'

import useLocation from 'wouter/use-location'

import Landing from './routes/landing'
import Signin from './routes/signin'
import Signup from './routes/signup'
import NotFound from './routes/not-found'
import User from './routes/user'
import MyBook from './routes/my-book'

const Pages = () => {
    const [location] = useLocation()

    switch (location) {
        case '/':
            return <Landing />

        case '/signin':
            return <Signin />

        case '/signup':
            return <Signup />

        case '/me':
            return <User />

        case '/my-book':
            return <MyBook />

        default:
            return <NotFound />
    }
}

export default Pages
