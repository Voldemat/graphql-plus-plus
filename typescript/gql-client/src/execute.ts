import { z } from 'zod/v4'
import { ClientConfig, Operation } from './types.js'

export interface ExecuteResult<T extends Operation> {
    result: z.infer<T['resultSchema']>
    response: Response
}

export async function execute<TContext, T extends Operation>(
    config: ClientConfig<TContext>,
    operation: T,
    variables: z.infer<T['variablesSchema']>,
): Promise<ExecuteResult<T>> {
    for (const middleware of config.middlewares.beforeSerialization) {
        [operation, variables] = await middleware(
            config.context,
            operation,
            variables
        )
    }
    let init = await config.serializer.serializeRequest(
        config.context, operation, variables
    )
    for (const middleware of config.middlewares.afterSerialization) {
        init = await middleware(config.context, operation, variables, init)
    }
    let response = await config.fetcher(init)
    for (const middleware of config.middlewares.beforeParsing) {
        response = await middleware(
            config.context, operation, variables, init, response
        )
    }
    let result = await config.parser.parseBody(
        config.context, operation, response
    )
    for (const middleware of config.middlewares.afterParsing) {
        result = await middleware(
            config.context,
            operation,
            variables,
            init,
            response,
            result
        )
    }
    return { result, response }
}

export type Executor = <T extends Operation>(
    operation: T,
    variables: z.infer<T['variablesSchema']>
) => Promise<ExecuteResult<T>>
export function bindConfigToExecute<TContext>(
    config: ClientConfig<TContext>
): Executor {
    return (operation, variables) => execute(config, operation, variables)
}
