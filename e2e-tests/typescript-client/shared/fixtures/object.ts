import { beforeAll } from 'bun:test'
import { createObjectProxy } from "./proxy"

export function useObjectFixture<T extends object>(
    callback: () => Promise<T>
): T {
    let obj: T

    beforeAll(async () => {
        obj = await callback()
    })
    
    return createObjectProxy<T>(() => obj)
}
