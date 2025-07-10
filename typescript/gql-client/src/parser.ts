import { z } from 'zod/v4';
import { ClientParser, Operation } from './types.js';

interface CreateParserOptions<TContext> {
    onErrors: <T extends Operation>(
        context: TContext,
        operation: T,
        response: Response,
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        errors: any[]
    ) => Promise<void> | void
}

const defaultParserOptions: CreateParserOptions<unknown> = {
    onErrors(_, __, ___, errors) {
        throw new Error(JSON.stringify(errors))
    },
}

export function createParser<TContext>(
    options: CreateParserOptions<TContext> = defaultParserOptions
): ClientParser<TContext> {
    return {
        parseBody: async <T extends Operation>(
            context: TContext,
            operation: T,
            response: Response
        ) => {
            const json = await response.json()
            if (json.errors) {
                await options.onErrors(
                    context, operation, response, json.errors
                )
            }
            return operation.resultSchema.parse(
                json.data
            ) as z.infer<T['resultSchema']>
        },
    }
}
