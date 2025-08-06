/* eslint-disable max-lines */
import {
    ClientConfig,
    OperationResult,
    OperationVariables,
    RequestContext,
    SubOpAsyncIterable,
    SubscriptionOperation,
    SyncOperation,
} from './types/index.js'
import {
    AfterParsingSubscriptionMiddlewareOptions,
    AfterParsingSyncMiddlewareOptions
} from './types/middlewares/parsing.js'

export interface ExecuteResult<TResult> {
    result: TResult
    response: Response
}

export class Executor<TClientContext, TRequestContext extends RequestContext> {
    constructor(
        private readonly config: ClientConfig<TClientContext, TRequestContext>
    ) { }

    async executeSync<T extends SyncOperation<unknown, unknown>>(
        operation: T,
        variables: OperationVariables<T>,
        requestContext: TRequestContext
    ): Promise<ExecuteResult<OperationResult<T>>> {
        for (const middleware of this.config.middlewares.beforeSerialization) {
            [operation, variables] = await middleware({
                clientContext: this.config.context,
                requestContext,
                operation,
                variables
            })
        }
        let init = await this.config.serializer.serializeRequest({
            clientContext: this.config.context,
            requestContext,
            operation,
            variables
        })
        for (
            const middleware of
            this.config.middlewares.afterSerialization.sync
        ) {
            init = await middleware({
                clientContext: this.config.context,
                requestContext,
                operation,
                variables,
                init
            })
        }
        let response = await this.config.fetcher(init)
        for (const middleware of this.config.middlewares.beforeParsing.sync) {
            response = await middleware({
                clientContext: this.config.context,
                requestContext,
                operation,
                variables,
                init,
                response
            })
        }
        let result = await this.config.parser.parseBodySync({
            clientContext: this.config.context,
            requestContext,
            operation,
            response
        })
        for (const middleware of this.config.middlewares.afterParsing.sync) {
            result = await middleware({
                clientContext: this.config.context,
                requestContext,
                operation,
                variables,
                init,
                response,
                result
            } as AfterParsingSyncMiddlewareOptions<
                TClientContext,
                TRequestContext,
                T
            >)
        }
        return { result, response }
    }

    async executeSubscription<
        T extends SubscriptionOperation<unknown, unknown>
    >(
        operation: T,
        variables: OperationVariables<T>,
        requestContext: TRequestContext,
        controller: AbortController
    ): Promise<ExecuteResult<SubOpAsyncIterable<OperationResult<T>>>> {
        for (const middleware of this.config.middlewares.beforeSerialization) {
            [operation, variables] = await middleware({
                clientContext: this.config.context,
                requestContext,
                operation,
                variables
            })
        }
        let init = await this.config.serializer.serializeRequest({
            clientContext: this.config.context,
            requestContext,
            operation,
            variables
        })
        for (
            const middleware of
            this.config.middlewares.afterSerialization.subscription
        ) {
            init = await middleware({
                clientContext: this.config.context,
                requestContext,
                operation,
                variables,
                init,
                controller
            })
        }
        let response = await this.config.fetcher(init)
        for (
            const middleware of
            this.config.middlewares.beforeParsing.subscription
        ) {
            response = await middleware({
                clientContext: this.config.context,
                requestContext,
                operation,
                variables,
                init,
                response,
                controller
            })
        }
        let result = await this.config.parser.parseBodySubscription({
            clientContext: this.config.context,
            requestContext,
            operation,
            response,
            controller
        })
        for (
            const middleware of
            this.config.middlewares.afterParsing.subscription
        ) {
            result = await middleware({
                clientContext: this.config.context,
                requestContext,
                operation,
                variables,
                init,
                response,
                result,
                controller
            } as AfterParsingSubscriptionMiddlewareOptions<
                TClientContext,
                TRequestContext,
                T
            >)
        }
        return { result, response }
    }
}
