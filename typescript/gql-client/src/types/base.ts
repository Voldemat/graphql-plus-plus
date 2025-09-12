import { z } from 'zod/v4'

export type SchemaForInput<T> = z.ZodType<unknown, T>;
export type SchemaForOutput<T> = z.ZodType<T>;

interface BaseOperation<V, R> {
    name: string
    document: string
    variablesSchema: SchemaForInput<V>
    resultSchema: SchemaForOutput<R>
    hash: string
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
