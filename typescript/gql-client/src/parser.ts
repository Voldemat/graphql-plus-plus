import { z } from 'zod/v4';
import { ClientParser, Operation, RequestContext } from './types/index.js';
import { ClientParserParseBodyOptions } from './types/parser.js';

interface CreateParserOptions<
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
                TClientContext, TRequestContext, T
            >
        ) => {
            const json = await options.response.json()
            if (json.errors) {
                await parserOptions.onErrors(options, json.errors)
            }
            return options.operation.resultSchema.parse(
                json.data
            ) as z.infer<T['resultSchema']>
        },
    }
}
