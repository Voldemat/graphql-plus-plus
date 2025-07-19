import {
    ClientParser,
    Operation,
    RequestContext,
    ClientParserParseBodyOptions,
    SubOpAsyncIterable,
    OperationResult,
} from './types/index.js';
import assert from 'assert';

export interface CreateParserOptions<
    TClientContext,
    TRequestContext extends RequestContext
> {
    onErrors: <T extends Operation<unknown, unknown>>(
        options: ClientParserParseBodyOptions<
            TClientContext, TRequestContext, T
        >,
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        errors: any[]
    ) => Promise<void> | void
}

const defaultParserOptions: CreateParserOptions<unknown, RequestContext> = {
    onErrors(_, errors) {
        throw new Error(JSON.stringify(errors))
    },
}

export function createParser<
    TClientContext,
    TRequestContext extends RequestContext
>(
    parserOptions: CreateParserOptions<
        TClientContext, TRequestContext
    > = defaultParserOptions
): ClientParser<TClientContext, TRequestContext> {
    return {
        parseBody: async <T extends Operation<unknown, unknown>>(
            options: ClientParserParseBodyOptions<
                TClientContext,
                TRequestContext,
                T
            >
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
        ): Promise<any> => {
            type TResult = OperationResult<T>
            if (options.operation.type === 'SUBSCRIPTION') {
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
                const close = () => shouldClose = true
                const stream = async function*() {
                    while (!shouldClose) {
                        const readResult = await reader.read()
                        if (readResult.done || shouldClose) break
                        const lines = decoder.decode(readResult.value)
                            .split('\n')
                        for (const line of lines.filter(c => c !== '')) {
                            if (shouldClose) return
                            const [name, value] = line.split(/:(.*)/s, 2)
                            if (name !== 'data') continue
                            yield options.operation.resultSchema.parse(
                                JSON.parse(value).data
                            )
                        }
                    }
                }()
                return { stream, close } as SubOpAsyncIterable<TResult>
            }
            const json = await options.response.json()
            if (json.errors) {
                await parserOptions.onErrors(options, json.errors)
            }
            return options.operation.resultSchema.parse(json.data) as TResult
        },
    }
}
