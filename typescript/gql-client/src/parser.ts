import { z } from 'zod/v4';
import { ClientParser, Operation } from './types.js';

export function createParser<TContext>(): ClientParser<TContext> {
    return {
        parseBody: async <T extends Operation>(
            _: TContext,
            operation: T,
            response: Response
        ) => {
            return operation.resultSchema.parse(
                await response.json()
            ) as z.infer<T['resultSchema']>
        },
    }
}
