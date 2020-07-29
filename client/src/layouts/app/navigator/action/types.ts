import { FunctionComponent } from 'preact'

interface ActionProps {
    children: string,
    href: string
}

type ActionComponent = FunctionComponent<ActionProps>
export default ActionComponent