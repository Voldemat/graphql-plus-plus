import { z } from 'zod/v4'
import { ClientConfig, Operation, RequestContext } from './types/index.js'

export interface ExecuteResult<TResult> {
    result: TResult
    response: Response
}

export async function execute<
    TClientContext,
    TRequestContext extends RequestContext,
    T extends Operation
>(
    config: ClientConfig<TClientContext, TRequestContext>,
    operation: T,
    variables: z.infer<T['variablesSchema']>,
    requestContext: TRequestContext
): Promise<ExecuteResult<z.infer<T['resultSchema']>>> {
    for (const middleware of config.middlewares.beforeSerialization) {
        [operation, variables] = await middleware({
            clientContext: config.context,
            requestContext,
            operation,
            variables
        })
    }
    let init = await config.serializer.serializeRequest({
        clientContext: config.context,
        requestContext,
        operation,
        variables
    })
    for (const middleware of config.middlewares.afterSerialization) {
        init = await middleware({
            clientContext: config.context,
            requestContext,
            operation,
            variables,
            init
        })
    }
    let response = await config.fetcher(init)
    for (const middleware of config.middlewares.beforeParsing) {
        response = await middleware({
            clientContext: config.context,
            requestContext,
            operation,
            variables,
            init,
            response
        })
    }
    let result = await config.parser.parseBody({
        clientContext: config.context,
        requestContext,
        operation,
        response
    })
    for (const middleware of config.middlewares.afterParsing) {
        result = await middleware({
            clientContext: config.context,
            requestContext,
            operation,
            variables,
            init,
            response,
            result
        })
    }
    return { result, response }
}

export type Executor<TRequestContext extends RequestContext> =
    <T extends Operation>(
        operation: T,
        variables: z.infer<T['variablesSchema']>,
        context: TRequestContext
    ) => Promise<ExecuteResult<z.infer<T['resultSchema']>>>
export function bindConfigToExecute<
    TClientContext,
    TRequestContext extends RequestContext
>(
    config: ClientConfig<TClientContext, TRequestContext>
): Executor<TRequestContext> {
    return (operation, variables, requestContext) =>
        execute(config, operation, variables, requestContext)
}
