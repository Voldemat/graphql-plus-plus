import type {
    SubscriptionOperation,
    SyncOperation,
    RequestContext,
    Operation,
} from './base.js'
import { OperationResult, PromiseOrValue } from './utils.js'

export interface ClientParserParseBodyOptions<
    TClientContext,
    TRequestContext extends RequestContext,
    TOperation
> {
    clientContext: TClientContext
    requestContext: TRequestContext
    operation: TOperation
    response: Response
}

export type SubOpAsyncIterable<TResult> = AsyncIterable<TResult, void, unknown>

export type ParseBodyFuncType<
    TClientContext,
    TRequestContext extends RequestContext
> = {
    <TSyncOp extends SyncOperation>(
        options: ClientParserParseBodyOptions<
            TClientContext,
            TRequestContext,
            TSyncOp
        >
    ): PromiseOrValue<OperationResult<TSyncOp>>
    <TSubOp extends SubscriptionOperation>(
        options: ClientParserParseBodyOptions<
            TClientContext,
            TRequestContext,
            TSubOp
        >
    ): PromiseOrValue<SubOpAsyncIterable<OperationResult<TSubOp>>>
    <TOp extends Operation>(
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

