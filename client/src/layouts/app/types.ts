import { FunctionComponent, VNode } from 'preact'

interface AppLayoutProps {
    children: VNode
}

type AppLayoutComponent = FunctionComponent<AppLayoutProps>
export default AppLayoutComponent