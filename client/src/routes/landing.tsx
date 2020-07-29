import { h } from 'preact'
import { useState } from 'preact/hooks'

import { useStoreon } from 'storeon/preact'
import { UserStore, UserEvent } from '../stores/types'

import useSWR from 'swr'
import graphqlRequest, { graphqlBase } from '../libs/graphql'

import BookDisplay from '../component/book'
import { Book } from '../component/book/types'

import '../styles/landing.styl'
import { Link } from 'wouter/preact'

const Landing = () => {
    let { username } = useStoreon<UserStore, UserEvent>('username')

    let [localBorrowed, updateBorrow] = useState<string[]>([])

    if (!username)
        return (
            <section id="book-list" class="-error">
                <h3 class="title">Night Library</h3>
                <p class="detail">
                    Online library management system written in Actix Web.
                </p>
                <Link href="/signin">
                    <a class="action">Signin</a>
                </Link>
            </section>
        )

    let { data: books, error } = useSWR(
        [
            'listBook',
            `{
              listBook {
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

    let { data: borrowed, error: borrowedError } = useSWR(
        [
            'listBorrowed',
            `{
              listBorrowed {
                data {
                  id
                }
              }
            }`
        ],
        graphqlRequest
    )

    if (error || borrowedError)
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

    if (!books || !borrowed)
        return (
            <section id="book-list">
                <h1 class="header">Book List</h1>
                {Array(8)
                    .fill(null)
                    .map((_, index) => (
                        <BookDisplay key={index} preload />
                    ))}
            </section>
        )

    let borrowedIdList = borrowed.map(({ id }: { id: string }) => id)

    let returnBook = async (id: string) => {
        let { data: borrowedList, success } = await graphqlBase(
            'borrowBooks',
            `mutation {
              borrowBooks(books: ["${id}"]) {
                data
                success
              }
            }`
        )

        if (!success) return console.log('Something went wrong')

        updateBorrow(borrowedList)
    }

    return (
        <section id="book-list">
            <h1 class="header">Book List</h1>
            {books.map((book: Book) => {
                let isBorrowed =
                    borrowedIdList.length || localBorrowed.length
                        ? borrowedIdList.includes(book.id) ||
                          localBorrowed.includes(book.id)
                        : false

                return (
                    <BookDisplay
                        key={book.id}
                        {...{ book }}
                        style={isBorrowed ? { opacity: 0.5 } : {}}
                        action={isBorrowed ? 'Borrowed' : 'Borrow'}
                        onClick={returnBook}
                        disabled={isBorrowed}
                    />
                )
            })}
        </section>
    )
}

export default Landing
