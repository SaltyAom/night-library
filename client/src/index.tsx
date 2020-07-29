  
import { h } from 'preact'
import { useEffect } from 'preact/hooks'

import store from './stores'
import { StoreContext } from 'storeon/preact'

import AppLayout from './layouts/app'
import RefreshProvider from './layouts/refresh-provider'

import Router from './router'

import './styles/init.styl'

const App = () => {
    useEffect(() => {
        document.addEventListener('touchstart', () => null, { passive: true })
    }, [])

    return (
        <main>
            <StoreContext.Provider value={store}>
                <RefreshProvider>
                    <AppLayout>
                        <Router />
                    </AppLayout>
                </RefreshProvider>
            </StoreContext.Provider>
        </main>
    )
}

export default App