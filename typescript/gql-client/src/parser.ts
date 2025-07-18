import { z } from 'zod/v4';
import { ClientParser, Operation, RequestContext } from './types/index.js';
import { ClientParserParseBodyOptions } from './types/parser.js';
import assert from 'assert';

export interface CreateParserOptions<
    TClientContext,
    TRequestContext extends RequestContext
> {
    onErrors: <T extends Operation>(
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
        parseBody: async <T extends Operation>(
            options: ClientParserParseBodyOptions<
                TClientContext,
                TRequestContext,
                T
            >
        ) => {
            type TResult = z.infer<T['resultSchema']>
            if (options.operation.type === 'SUBSCRIPTION') {
                if (options.response.status !== 200) {
                    throw new Error(await options.response.text())
                }
                assert(
                    options.response.headers.get('content-type') ===
                        'text/event-stream'
                )
                const stream = options.response.body
                assert(stream !== null)
                const reader = stream.getReader()
                const decoder = new TextDecoder()
                return async function*() {
                    while (true) {
                        const readResult = await reader.read()
                        if (readResult.done) break
                        const lines = decoder.decode(readResult.value)
                            .split('\n')
                        for (const line of lines.filter(c => c!== '')) {
                            const [name, value] = line.split(/:(.*)/s, 2)
                            if (name !== 'data') continue
                            yield options.operation.resultSchema.parse(
                                JSON.parse(value).data
                            ) as TResult
                        }
                    }
                }()
            }
            const json = await options.response.json()
            if (json.errors) {
                await parserOptions.onErrors(options, json.errors)
            }
            return options.operation.resultSchema.parse(json.data) as TResult
        },
    }
}
