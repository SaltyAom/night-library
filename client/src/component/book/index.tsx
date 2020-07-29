import { h } from 'preact'

import BookComponent, { Book as BookProps } from './types'

import './book.styl'

const Book: BookComponent = ({
    book,
    preload = false,
    hiddenPrice = false,
    onClick = () => null,
    action = 'Borrow',
    style = {},
    disabled = false
}) => {
    if (preload)
        return (
            <article class="book -preload">
                <section class="content">
                    <h1 class="title" />
                    <p class="author" />
                    {hiddenPrice || <p class="price" />}
                </section>
                <button class="lend" disabled={true} />
            </article>
        )

    let { id, title, author, price } = book as BookProps

    return (
        <article class={`book ${id}`} style={style}>
            <section class="content">
                <h1 class="title">{title}</h1>
                <p class="author">{author}</p>
                {hiddenPrice || <p class="price">{price}</p>}
            </section>
            <button
                class={`lend ${disabled ? '-disabled' : ''}`}
                onClick={() => onClick(id)}
                disabled={disabled}
            >
                {action}
            </button>
        </article>
    )
}

export default Book
