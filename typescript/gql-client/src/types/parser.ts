import type {
    SubscriptionOperation,
    SyncOperation,
    RequestContext,
    Operation,
    OperationResult,
} from './base.js'
import { PromiseOrValue } from './utils.js'

export interface ClientParserParseBodySyncOptions<
    TClientContext,
    TRequestContext extends RequestContext,
    TOperation extends Operation<unknown, unknown>
> {
    clientContext: TClientContext
    requestContext: TRequestContext
    operation: TOperation
    response: Response
}

export interface ClientParserParseBodySubscriptionOptions<
    TClientContext,
    TRequestContext extends RequestContext,
    TOperation extends Operation<unknown, unknown>
> {
    clientContext: TClientContext
    requestContext: TRequestContext
    operation: TOperation
    response: Response
    controller: AbortController
}

export interface SubOpAsyncIterable<TResult> extends Disposable {
    stream: AsyncIterable<TResult, void, unknown>
    close: () => void
}

export type ParseBodySyncFuncType<
    TClientContext,
    TRequestContext extends RequestContext
> = <TSyncOp extends SyncOperation<unknown, unknown>>(
    options: ClientParserParseBodySyncOptions<
        TClientContext,
        TRequestContext,
        TSyncOp
    >
) => PromiseOrValue<OperationResult<TSyncOp>>

export type ParseBodySubscriptionFuncType<
    TClientContext,
    TRequestContext extends RequestContext
> = <TSubOp extends SubscriptionOperation<unknown, unknown>>(
    options: ClientParserParseBodySubscriptionOptions<
        TClientContext,
        TRequestContext,
        TSubOp
    >
) => PromiseOrValue<SubOpAsyncIterable<OperationResult<TSubOp>>>

export interface ClientParser<
    TClientContext,
    TRequestContext extends RequestContext
> {
    parseBodySync: ParseBodySyncFuncType<TClientContext, TRequestContext>
    parseBodySubscription: ParseBodySubscriptionFuncType<
        TClientContext, TRequestContext
    >
}

