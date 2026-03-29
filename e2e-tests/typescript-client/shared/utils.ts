export async function asyncTimeout (timeoutMs: number): Promise<void> {
    await new Promise<void>(resolve => setTimeout(resolve, timeoutMs))
}

async function awaitTimeout (
    delay: number,
    reason: string = 'Timeout exceeded'
): Promise<never> {
    return await new Promise<never>((_, reject) =>
        setTimeout(() => { reject(reason) }, delay))
}

export async function awaitWithTimeout<T> (
    promise: Promise<T>,
    delay: number,
    reason?: string
): Promise<T> {
    return await Promise.race([
        promise,
        awaitTimeout(delay, reason)
    ])
}
