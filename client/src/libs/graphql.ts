const graphqlRequest = async (
    operationName: string,
    query: string
): Promise<any> => {
    if (typeof window === 'undefined') return []

    let { data } = await graphqlBase(operationName, query)

    return data
}

const graphqlBase = async (
    operationName: string,
    query: string
): Promise<any> => {
    if (typeof window === 'undefined') return []

    const response = await fetch('http://localhost/graphql', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        mode: 'cors',
        credentials: 'include',
        body: JSON.stringify({
            query
        })
    })

    const { data } = await response.json()

    return data[operationName]
}

export { graphqlBase }

export default graphqlRequest
