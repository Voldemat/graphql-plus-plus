import { Operation, SubscriptionOperation, SyncOperation } from './base.js'

export type PromiseOrValue<T> = Promise<T> | T
export type OpResultBasedOnOp<T extends Operation<unknown, unknown>> =
    T extends SyncOperation<unknown, infer R> ? R :
    T extends SubscriptionOperation<unknown, infer R> ? AsyncIterable<R> :
    never
