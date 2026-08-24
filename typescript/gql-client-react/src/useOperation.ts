import hash, { type NotUndefined } from 'object-hash'
import { useEffect, useMemo, useState } from 'react'
import { loadingState } from './loading-state.js'
import type {
    ExecuteResult,
    IExecutor,
    OperationResult,
    OperationVariables,
    RequestContext,
    SyncOperation,
} from './types.js'

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
    | OperationLoadingState
    | OperationSuccessState<TResult>
    | OperationFailureState
export type UseOperationHookReturnType<TResult> = OperationState<TResult>
export function useOperation<
    TExecutor extends IExecutor<TRequestContext>,
    TRequestContext extends RequestContext,
    TOperation extends SyncOperation<unknown, unknown>,
>(
    executor: TExecutor,
    operation: TOperation,
    variables: OperationVariables<TOperation>,
    requestContext: TRequestContext,
): UseOperationHookReturnType<OperationResult<TOperation>> {
    const [state, setState] =
        useState<OperationState<OperationResult<TOperation>>>(loadingState)
    const memoizedVariables = useMemo(
        () => variables,
        // oxlint-disable-next-line exhaustive-deps,use-memo
        [hash(variables as NotUndefined)],
    )
    const memoizedRequestContext = useMemo(
        () => requestContext,
        // oxlint-disable-next-line exhaustive-deps,use-memo
        [hash(requestContext)],
    )
    useEffect(() => {
        executor
            .executeSync(operation, memoizedVariables, memoizedRequestContext)
            .then((result) => setState({ state: 'success', ...result }))
            .catch((error) => setState({ state: 'failure', error }))
        return () => setState(loadingState)
    }, [
        setState,
        executor,
        operation,
        memoizedVariables,
        memoizedRequestContext,
    ])
    return state
}
