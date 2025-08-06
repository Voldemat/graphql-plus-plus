import {
    OperationResult,
    RequestContext,
    SubscriptionOperation
} from '@/types/base.js'
import {
    ClientParserParseBodySubscriptionOptions,
    SubOpAsyncIterable
} from '@/types/parser.js'
import assert from 'assert'

export function buildParseBodySubscriptionFunc<
    TClientContext,
    TRequestContext extends RequestContext
>() {
    return async <T extends SubscriptionOperation<unknown, unknown>>(
        options: ClientParserParseBodySubscriptionOptions<
            TClientContext,
            TRequestContext,
            T
        >
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
    ): Promise<any> => {
        type TResult = OperationResult<T>
        if (options.response.status !== 200) {
            throw new Error(await options.response.text())
        }
        assert(
            options.response.headers.get('content-type') ===
            'text/event-stream'
        )
        const rStream = options.response.body
        assert(rStream !== null)
        const reader = rStream.getReader()
        const decoder = new TextDecoder()
        let shouldClose = false
        type ReadResult = Awaited<ReturnType<typeof reader.read>>
        let resolve: (v: ReadResult) => void
        const signal = new Promise<ReadResult>(
            res => resolve = res
        )
        const close = () => {
            shouldClose = true
            resolve({ done: true, value: undefined })
            options.controller.abort()
        }
        const stream = async function*() {
            while (!shouldClose) {
                const readResult = await Promise.race([
                    reader.read(),
                    signal
                ])
                if (readResult.done) return
                const lines = decoder.decode(readResult.value)
                    .split('\n')
                for (const line of lines.filter(c => c !== '')) {
                    if (shouldClose) break
                    const [name, value] = line.split(/:(.*)/s, 2)
                    if (name !== 'data') continue
                    yield options.operation.resultSchema.parse(
                        JSON.parse(value).data
                    )
                }
            }
        }()
        return {
            stream,
            close,
            [Symbol.dispose]: close
        } as SubOpAsyncIterable<TResult>
    }
}
