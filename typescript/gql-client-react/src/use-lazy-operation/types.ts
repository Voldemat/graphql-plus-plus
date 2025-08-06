import type { RequestContext } from '../types.js'
import {
    type OperationFailureState,
    type OperationState,
    type OperationSuccessState
} from '../useOperation.jsx'

export interface LazyOperationInitialState { state: 'initial' }
export type LazyOperationState<TResult> =
    OperationState<TResult> | LazyOperationInitialState

export type LazyOperationExecuteReturnType<TResult> = Promise<
    OperationSuccessState<TResult> |
    OperationFailureState
>

export type UseLazyOperationReturnType<
    TVariables,
    TResult,
    TRequestContext extends RequestContext
> = [
    (
        variables: TVariables,
        requestContext: TRequestContext
    ) => LazyOperationExecuteReturnType<TResult>,
    {
        state: LazyOperationState<TResult>
        reset: () => void
    }
]
