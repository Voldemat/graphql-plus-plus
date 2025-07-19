/* eslint-disable max-lines */
import {
    ClientConfig,
    Operation,
    OperationResult,
    OperationVariables,
    RequestContext,
    SubOpAsyncIterable,
    SubscriptionOperation,
    SyncOperation,
} from './types/index.js'
import { AfterParsingMiddlewareOptions } from './types/middlewares.js'
import { OpResultBasedOnOp } from './types/utils.js'

export interface ExecuteResult<TResult> {
    result: TResult
    response: Response
}

export async function execute<
    TClientContext,
    TRequestContext extends RequestContext,
    T extends Operation<unknown, unknown>
>(
    config: ClientConfig<TClientContext, TRequestContext>,
    operation: T,
    variables: OperationVariables<T>,
    requestContext: TRequestContext
): Promise<ExecuteResult<OpResultBasedOnOp<T>>> {
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
        } as AfterParsingMiddlewareOptions<TClientContext, TRequestContext, T>)
    }
    return { result, response } as ExecuteResult<OpResultBasedOnOp<T>>
}

export type Executor<TRequestContext extends RequestContext> = {
    <TSyncOp extends SyncOperation<unknown, unknown>>(
        operation: TSyncOp,
        variables: OperationVariables<TSyncOp>,
        context: TRequestContext
    ): Promise<ExecuteResult<OperationResult<TSyncOp>>>
    <TOperation extends SubscriptionOperation<unknown, unknown>>(
        operation: TOperation,
        variables: OperationVariables<TOperation>,
        context: TRequestContext
    ): Promise<ExecuteResult<SubOpAsyncIterable<OperationResult<TOperation>>>>
}

export function bindConfigToExecute<
    TClientContext,
    TRequestContext extends RequestContext
>(
    config: ClientConfig<TClientContext, TRequestContext>
): Executor<TRequestContext> {
    return <T extends Operation<unknown, unknown>>(
        operation: T,
        variables: OperationVariables<T>,
        requestContext: TRequestContext
    ) => execute(
        config, operation, variables, requestContext
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
    ) as any
}
