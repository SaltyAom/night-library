import { h } from 'preact'
import { useState } from 'preact/hooks'

import { Link, Redirect } from 'wouter/preact'

import useSWR from 'swr'
import graphqlRequest, { graphqlBase } from '../libs/graphql'

import BookDisplay from '../component/book'
import { Book } from '../component/book/types'

import '../styles/landing.styl'
import { useStoreon } from 'storeon/preact'
import { UserStore, UserEvent } from '../stores/types'

const MyBook = () => {
    let { username, initial } = useStoreon<UserStore, UserEvent>(
        'username',
        'initial'
    )

    let { data: books, error } = useSWR(
        [
            'listBorrowed',
            `{
              listBorrowed {
                data {
                  id,
                  title,
                  author,
                  price
                }
              }
            }`
        ],
        graphqlRequest
    )

    let [returned, updateReturned] = useState<String[]>([])

    let returnBook = async (id: string) => {
        let { success } = await graphqlBase(
            'returnBooks',
            `mutation {
              returnBooks(books: ["${id}"]) {
                success
              }
            }`
        )

        if (!success) return console.log('Something went wrong')

        updateReturned([...returned, id])
    }

    if (error)
        return (
            <section id="book-list" class="-error">
                <h3 class="title">Failed to fetch</h3>
                <p class="detail">
                    There is a problem requesting an list of books.
                </p>
                <button class="action" onClick={() => window.location.reload()}>
                    Retry
                </button>
            </section>
        )

    if (!books || initial)
        return (
            <section id="book-list">
                <h1 class="header">My Book</h1>
                {Array(8)
                    .fill(null)
                    .map((_, index) => (
                        <BookDisplay key={index} preload hiddenPrice />
                    ))}
            </section>
        )

    if (!username) return <Redirect href="/signin" />

    if (!books.length || books.length === returned.length)
        return (
            <section id="book-list" class="-error">
                <h3 class="title">No Borrow</h3>
                <p class="detail">No book have been borrowed.</p>
                <Link href="/">
                    <button class="action -compact">Borrow some book</button>
                </Link>
            </section>
        )

    return (
        <section id="book-list">
            <h1 class="header">My Book</h1>
            {books
                .filter((book: Book) => !returned.includes(book.id))
                .map((book: Book) => (
                    <BookDisplay
                        key={book.id}
                        {...{ book }}
                        hiddenPrice
                        onClick={returnBook}
                        action="Return"
                    />
                ))}
        </section>
    )
}

export default MyBook
