import {
    OperationResult,
    OperationVariables,
    RequestContext,
    SubscriptionOperation,
    SyncOperation
} from './base.js';
import { SubOpAsyncIterable } from './parser.js';

export interface ExecuteResult<TResult> {
    result: TResult
    response: Response
}

export interface IExecutor<TRequestContext extends RequestContext> {
    executeSync<T extends SyncOperation<unknown, unknown>>(
        operation: T,
        variables: OperationVariables<T>,
        requestContext: TRequestContext
    ): Promise<ExecuteResult<OperationResult<T>>>
    executeSubscription<T extends SubscriptionOperation<unknown, unknown>>(
        operation: T,
        variables: OperationVariables<T>,
        requestContext: TRequestContext,
        controller: AbortController
    ): Promise<ExecuteResult<SubOpAsyncIterable<OperationResult<T>>>>
}
