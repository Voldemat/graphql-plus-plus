import type {
    SubscriptionOperation,
    SyncOperation,
    RequestContext,
    Operation,
    OperationResult,
} from './base.js'
import { PromiseOrValue } from './utils.js'

export interface ClientParserParseBodyOptions<
    TClientContext,
    TRequestContext extends RequestContext,
    TOperation extends Operation<unknown, unknown>
> {
    clientContext: TClientContext
    requestContext: TRequestContext
    operation: TOperation
    response: Response
}

export type SubOpAsyncIterable<TResult> = {
    stream: AsyncIterable<TResult, void, unknown>
    close: () => void
}

export type ParseBodyFuncType<
    TClientContext,
    TRequestContext extends RequestContext
> = {
    <TSyncOp extends SyncOperation<unknown, unknown>>(
        options: ClientParserParseBodyOptions<
            TClientContext,
            TRequestContext,
            TSyncOp
        >
    ): PromiseOrValue<OperationResult<TSyncOp>>
    <TSubOp extends SubscriptionOperation<unknown, unknown>>(
        options: ClientParserParseBodyOptions<
            TClientContext,
            TRequestContext,
            TSubOp
        >
    ): PromiseOrValue<SubOpAsyncIterable<OperationResult<TSubOp>>>
    <TOp extends Operation<unknown, unknown>>(
        options: ClientParserParseBodyOptions<
            TClientContext,
            TRequestContext,
            TOp
        >
    ): PromiseOrValue<OperationResult<TOp>> |
        PromiseOrValue<SubOpAsyncIterable<OperationResult<TOp>>>
}

export interface ClientParser<
    TClientContext,
    TRequestContext extends RequestContext
> {
    parseBody: ParseBodyFuncType<TClientContext, TRequestContext>
}

