import { h, Fragment } from "preact";

import Navigator from './navigator'

import AppLayoutComponent from './types'

import './app-layout.styl'

const AppLayout: AppLayoutComponent = ({ children }) => (
    <Fragment>
        <Navigator />
        <main id="main">
            {children}
        </main>
    </Fragment>
)

export default AppLayout