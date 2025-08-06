import {
    OperationResult,
    OperationVariables,
    RequestContext,
    SubscriptionOperation,
    SyncOperation
} from '../base.js'
import { SubOpAsyncIterable } from '../parser.js'
import { OpResultBasedOnOp, PromiseOrValue } from '../utils.js'

export type BeforeParsingSyncMiddleware<
    TClientContext,
    TRequestContext extends RequestContext
> = <T extends SyncOperation<unknown, unknown>>(options: {
    clientContext: TClientContext,
    requestContext: TRequestContext,
    operation: T,
    variables: OperationVariables<T>,
    init: RequestInit,
    response: Response
}) => PromiseOrValue<Response>

export type BeforeParsingSubscriptionMiddleware<
    TClientContext,
    TRequestContext extends RequestContext
> = <T extends SubscriptionOperation<unknown, unknown>>(options: {
    clientContext: TClientContext,
    requestContext: TRequestContext,
    operation: T,
    variables: OperationVariables<T>,
    init: RequestInit,
    response: Response,
    controller: AbortController
}) => PromiseOrValue<Response>

export interface AfterParsingSyncMiddlewareOptions<
    TClientContext,
    TRequestContext extends RequestContext,
    T extends SyncOperation<unknown, unknown>
> {
    clientContext: TClientContext,
    requestContext: TRequestContext,
    operation: T,
    variables: OperationVariables<T>,
    init: RequestInit,
    response: Response,
    result: OpResultBasedOnOp<T>
}

export interface AfterParsingSubscriptionMiddlewareOptions<
    TClientContext,
    TRequestContext extends RequestContext,
    T extends SubscriptionOperation<unknown, unknown>
> {
    clientContext: TClientContext,
    requestContext: TRequestContext,
    operation: T,
    variables: OperationVariables<T>,
    init: RequestInit,
    response: Response,
    result: OpResultBasedOnOp<T>
    controller: AbortController
}

export type AfterParsingSyncMiddleware<
    TClientContext,
    TRequestContext extends RequestContext
> = <TSyncOp extends SyncOperation<unknown, unknown>>(
    options: AfterParsingSyncMiddlewareOptions<
        TClientContext,
        TRequestContext,
        TSyncOp
    >
) => PromiseOrValue<OperationResult<TSyncOp>>

export type AfterParsingSubscriptionMiddleware<
    TClientContext,
    TRequestContext extends RequestContext
> = <TSubOp extends SubscriptionOperation<unknown, unknown>>(
    options: AfterParsingSubscriptionMiddlewareOptions<
        TClientContext,
        TRequestContext,
        TSubOp
    >
) => PromiseOrValue<SubOpAsyncIterable<OperationResult<TSubOp>>>
