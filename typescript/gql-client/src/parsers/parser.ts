import {
    ClientParser,
    Operation,
    RequestContext,
    OperationResult,
    ClientParserParseBodySyncOptions
} from '../types/index.js';
import { buildParseBodySubscriptionFunc } from './parseBodySubscription.js';

export interface CreateParserOptions<
    TClientContext,
    TRequestContext extends RequestContext
> {
    onErrors: <T extends Operation<unknown, unknown>>(
        options: ClientParserParseBodySyncOptions<
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
        parseBodySync: async <T extends Operation<unknown, unknown>>(
            options: ClientParserParseBodySyncOptions<
                TClientContext,
                TRequestContext,
                T
            >
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
        ): Promise<any> => {
            type TResult = OperationResult<T>
            const json = await options.response.json()
            if (json.errors) {
                await parserOptions.onErrors(options, json.errors)
            }
            return options.operation.resultSchema.parse(json.data) as TResult
        },
        parseBodySubscription: buildParseBodySubscriptionFunc<
            TClientContext,
            TRequestContext
        >()
    }
}
