import { FunctionComponent } from 'preact'

export interface Book {
    id: string
    title: string
    author: string
    price: number
}

export type Books = Book[]

export interface BookProps {
    book: Book
    preload?: boolean
    hiddenPrice?: boolean
    onClick?: (id: string) => any
    action?: string
    style?: any
    disabled?: boolean
}

export interface BookPropsPreload {
    book?: null,
    preload?: true
    hiddenPrice?: boolean
    onClick?: (id: string) => any
    action?: string
    style?: any
    disabled?: boolean
}

type BookComponent = FunctionComponent<BookProps | BookPropsPreload>
export default BookComponent