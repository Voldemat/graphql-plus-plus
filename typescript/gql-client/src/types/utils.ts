import { z } from 'zod/v4'
import { Operation, SubscriptionOperation, SyncOperation } from './base.js'

export type PromiseOrValue<T> = Promise<T> | T
export type OperationResult<T extends Operation> = z.infer<T['resultSchema']>
export type OperationVariables<T extends Operation> =
    z.infer<T['variablesSchema']>

export type OpResultBasedOnOp<T extends Operation> =
    T extends SyncOperation ? OperationResult<T> :
    T extends SubscriptionOperation ? AsyncIterable<OperationResult<T>> :
    never
