import { createStoreon } from 'storeon'

import user from './user'

const store = createStoreon<any>([user])
export default store
