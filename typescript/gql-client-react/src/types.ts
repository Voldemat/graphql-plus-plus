import { z } from 'zod/v4'

export type SchemaFor<T> = z.ZodType<T>;

interface BaseOperation<V, R> {
    name: string
    document: string
    variablesSchema: SchemaFor<V>
    resultSchema: SchemaFor<R>
}

export interface SyncOperation<V, R> extends BaseOperation<V, R> {
    type: 'QUERY' | 'MUTATION'
}

export interface SubscriptionOperation<V, R> extends BaseOperation<V, R> {
    type: 'SUBSCRIPTION'
}

export type Operation<V, R> = SyncOperation<V, R> | SubscriptionOperation<V, R>

export interface RequestContext {
    fetchOptions?: RequestInit
}

export type OperationVariables<T extends Operation<unknown, unknown>> =
    T extends Operation<infer V, unknown> ? V : never

export type OperationResult<T extends Operation<unknown, unknown>> =
    T extends Operation<unknown, infer R> ? R : never

export interface ExecuteResult<TResult> {
    result: TResult
    response: Response
}

export interface SubOpAsyncIterable<TResult> extends Disposable {
    stream: AsyncIterable<TResult, void, unknown>
    close: () => void
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
