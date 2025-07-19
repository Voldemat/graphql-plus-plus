import {
    Operation,
    SubscriptionOperation,
    SyncOperation,
    RequestContext,
    OperationVariables,
    OperationResult,
} from './base.js'
import { SubOpAsyncIterable } from './parser.js'
import { OpResultBasedOnOp, PromiseOrValue } from './utils.js'

export type BeforeSerializationMiddleware<
    TClientContext,
    TRequestContext extends RequestContext
> = <T extends Operation<unknown, unknown>>(options: {
    clientContext: TClientContext,
    requestContext: TRequestContext,
    operation: T,
    variables: OperationVariables<T>
}) => PromiseOrValue<[T, OperationVariables<T>]>

export type AfterSerializationMiddleware<
    TClientContext,
    TRequestContext extends RequestContext
> = <T extends Operation<unknown, unknown>>(options: {
    clientContext: TClientContext,
    requestContext: TRequestContext,
    operation: T,
    variables: OperationVariables<T>,
    init: RequestInit
}) => PromiseOrValue<RequestInit>

export type BeforeParsingMiddleware<
    TClientContext,
    TRequestContext extends RequestContext
> = <T extends Operation<unknown, unknown>>(options: {
    clientContext: TClientContext,
    requestContext: TRequestContext,
    operation: T,
    variables: OperationVariables<T>,
    init: RequestInit,
    response: Response
}) => PromiseOrValue<Response>

export interface AfterParsingMiddlewareOptions<
    TClientContext,
    TRequestContext extends RequestContext,
    T extends Operation<unknown, unknown>
> {
    clientContext: TClientContext,
    requestContext: TRequestContext,
    operation: T,
    variables: OperationVariables<T>,
    init: RequestInit,
    response: Response,
    result: OpResultBasedOnOp<T>
}
export type AfterParsingMiddleware<
    TClientContext,
    TRequestContext extends RequestContext
> = {
    <TSyncOp extends SyncOperation<unknown, unknown>>(
        options: AfterParsingMiddlewareOptions<
            TClientContext,
            TRequestContext,
            TSyncOp
        >
    ): PromiseOrValue<OperationResult<TSyncOp>>
    <TSubOp extends SubscriptionOperation<unknown, unknown>>(
        options: AfterParsingMiddlewareOptions<
            TClientContext,
            TRequestContext,
            TSubOp
        >
    ): PromiseOrValue<SubOpAsyncIterable<OperationResult<TSubOp>>>
    <TOp extends Operation<unknown, unknown>>(
        options: AfterParsingMiddlewareOptions<
            TClientContext,
            TRequestContext,
            TOp
        >
    ): PromiseOrValue<OperationResult<TOp>> |
        PromiseOrValue<SubOpAsyncIterable<OperationResult<TOp>>>
}
export interface ClientMiddlewaresConfig<
    TClientContext,
    TRequestContext extends RequestContext
> {
    beforeSerialization: BeforeSerializationMiddleware<
        TClientContext, TRequestContext
    >[]
    afterSerialization: AfterSerializationMiddleware<
        TClientContext, TRequestContext
    >[]
    beforeParsing: BeforeParsingMiddleware<TClientContext, TRequestContext>[]
    afterParsing: AfterParsingMiddleware<TClientContext, TRequestContext>[]
}
