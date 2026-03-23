import { beforeAll } from 'bun:test'
import { createObjectProxy } from './proxy'

export function useObject<T extends Exclude<object, unknown[]>> (
    callback: () => T
): T {
    let value: T

    beforeAll(() => {
        value = callback()
    })

    return createObjectProxy(() => value)
}
