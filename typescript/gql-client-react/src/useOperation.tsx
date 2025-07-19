import { useEffect, useMemo, useState } from 'react';
import type {
    ExecuteResult,
    Executor,
    OperationResult,
    OperationVariables,
    RequestContext,
    SyncOperation
} from './types.js';
import hash, { type NotUndefined } from 'object-hash'

export interface OperationLoadingState {
    state: 'loading'
}

export interface OperationSuccessState<TResult> extends ExecuteResult<TResult> {
    state: 'success'
}

export interface OperationFailureState {
    state: 'failure'
    error: Error
}

export type OperationState<TResult> =
    OperationLoadingState |
    OperationSuccessState<TResult> |
    OperationFailureState

export const loadingState =
    Object.freeze({ state: 'loading' } as const) satisfies OperationLoadingState

export function useOperation<
    T extends SyncOperation<unknown, unknown>,
    TRequestContext extends RequestContext
>(
    executor: Executor<TRequestContext>,
    operation: T,
    variables: OperationVariables<T>,
    requestContext: TRequestContext
): OperationState<OperationResult<T>> {
    const [state, setState] = useState<OperationState<OperationResult<T>>>(
        loadingState
    )
    const memoizedVariables = useMemo(
        () => variables, [hash(variables as NotUndefined)]
    )
    const memoizedRequestContext = useMemo(
        () => requestContext, [hash(requestContext)]
    )
    useEffect(
        () => {
            executor(operation, variables, requestContext)
                .then(result => setState({ state: 'success', ...result }))
                .catch(error => setState({ state: 'failure', error }))
            return () => setState(loadingState)
        },
        [
            setState,
            executor,
            operation,
            memoizedVariables,
            memoizedRequestContext
        ]
    )
    return state
}
