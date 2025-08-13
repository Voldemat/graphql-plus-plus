import {
    Operation,
    OperationVariables,
    RequestContext,
    SubscriptionOperation,
    SyncOperation
} from './base.js'
import { ClientMiddlewaresConfig } from './middlewares/config.js'
import { ClientParser } from './parser.js'
import { ClientSerializer } from './serializer.js'

interface RetryConfigOptions<
    TClientContext,
    TRequestContext extends RequestContext,
    OperationType extends Operation<unknown, unknown>
> {
    context: TClientContext,
    requestContext: TRequestContext
    operation: OperationType
    variables: OperationVariables<OperationType>
}

interface RetryConfig<TClientContext, TRequestContext extends RequestContext> {
    shouldSyncRetry: (
        error: unknown,
        iteration: number,
        options: RetryConfigOptions<
            TClientContext,
            TRequestContext,
            SyncOperation<unknown, unknown>
        >
    ) => boolean
    shouldSubscriptionRetry: (
        error: unknown,
        iteration: number,
        options: RetryConfigOptions<
            TClientContext,
            TRequestContext,
            SubscriptionOperation<unknown, unknown>
        >
    ) => boolean
}

export interface ClientConfig<
    TClientContext,
    TRequestContext extends RequestContext
> {
    context: TClientContext
    retryConfig: RetryConfig<TClientContext, TRequestContext>
    parser: ClientParser<TClientContext, TRequestContext>
    serializer: ClientSerializer<TClientContext, TRequestContext>
    middlewares: ClientMiddlewaresConfig<TClientContext, TRequestContext>
    fetcher: (init: RequestInit) => Promise<Response>
}
