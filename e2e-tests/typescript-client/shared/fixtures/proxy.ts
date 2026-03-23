import { beforeAll } from 'bun:test'

function createProxy<T extends object> (
    getter: () => T, defaultValue: T = {} as T
): T {
    return new Proxy<T>(
        defaultValue,  
        {
            get: (_, p) => {
                // eslint-disable-next-line @typescript-eslint/no-explicit-any
                return (getter() as any)[p]
            },
            set: () => {
                throw new Error('Fixtures objects are readonly')
            },
            ownKeys: () => {
                const value = getter()
                const keys = Object.keys(value)
                if (Array.isArray(value)) {
                    keys.push('length')
                }
                return keys
            },
            getOwnPropertyDescriptor: (_, key) => {
                return Object.getOwnPropertyDescriptor(getter(), key)
            },
            deleteProperty: () => {
                throw new Error('Fixtures objects are readonly')
            },
            has: (_, key) => {
                // eslint-disable-next-line @typescript-eslint/no-explicit-any
                return Object.hasOwn(getter() as any, key)
            }
        }
    )
}
export function createObjectProxy<T extends Exclude<object, unknown[]>> (
    getter: () => T
): T {
    return createProxy<T>(getter)
}
export function createArrayProxy<T extends unknown[]> (
    getter: () => T
): T {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    return createProxy<T>(getter, [] as any)
}

export function buildObjectFixture<
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    Func extends (...args: any[]) => Promise<object>
> (func: Func): (...args: Parameters<Func>) => Awaited<ReturnType<Func>> {
    return (...args) => {
        let state: Awaited<ReturnType<Func>>

        beforeAll(async () => {
            state = await func(...args) as Awaited<ReturnType<Func>>
        })

        return createObjectProxy(() => state)
    }
}

export function buildArrayFixture<
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    Func extends (...args: any[]) => Promise<unknown[]>
> (func: Func): (...args: Parameters<Func>) => Awaited<ReturnType<Func>> {
    return (...args) => {
        let state: Awaited<ReturnType<Func>>

        beforeAll(async () => {
            state = await func(...args) as Awaited<ReturnType<Func>>
        })

        return createArrayProxy(() => state)
    }
}
