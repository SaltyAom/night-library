import { VNode, FunctionComponent } from 'preact'

export interface RefreshProviderProps {
    children: VNode
}

type RefreshProviderComponent = FunctionComponent<RefreshProviderProps>
export default RefreshProviderComponent