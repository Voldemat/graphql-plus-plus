import {
    useCallback,
    useState,
    type Dispatch,
    type SetStateAction,
} from 'react'
import { loadingState } from '../loading-state.js'
import type {
    IExecutor,
    OperationResult,
    OperationVariables,
    RequestContext,
    SyncOperation,
} from '../types.js'
import { type OperationState } from '../useOperation.jsx'
import type {
    LazyOperationExecuteReturnType,
    LazyOperationInitialState,
    LazyOperationState,
    UseLazyOperationHookReturnType,
} from './types.js'

const lazyInitialState = Object.freeze({
    state: 'initial',
} as const) satisfies LazyOperationInitialState

async function execute<
    TExecutor extends IExecutor<TRequestContext>,
    TRequestContext extends RequestContext,
    TOperation extends SyncOperation<unknown, unknown>,
>(
    executor: TExecutor,
    operation: TOperation,
    variables: OperationVariables<TOperation>,
    requestContext: TRequestContext,
    setState: Dispatch<
        SetStateAction<LazyOperationState<OperationResult<TOperation>>>
    >,
): LazyOperationExecuteReturnType<OperationResult<TOperation>> {
    let newState: OperationState<OperationResult<TOperation>>
    try {
        const result = await executor.executeSync(
            operation,
            variables,
            requestContext,
        )
        newState = { state: 'success', ...result }
    } catch (error: unknown) {
        newState = { state: 'failure', error: error as Error }
    }
    setState(newState)
    return newState
}

export function useLazyOperation<
    TExecutor extends IExecutor<TRequestContext>,
    TRequestContext extends RequestContext,
    TOperation extends SyncOperation<unknown, unknown>,
>(
    executor: TExecutor,
    operation: TOperation,
): UseLazyOperationHookReturnType<
    TRequestContext,
    OperationVariables<TOperation>,
    OperationResult<TOperation>
> {
    const [state, setState] =
        useState<LazyOperationState<OperationResult<TOperation>>>(
            lazyInitialState,
        )
    const executeCallback = useCallback(
        (
            variables: OperationVariables<TOperation>,
            requestContext: TRequestContext,
        ) => {
            setState(loadingState)
            return execute(
                executor,
                operation,
                variables,
                requestContext,
                setState,
            )
        },
        [setState, executor, operation],
    )
    return [executeCallback, { state, reset: () => setState(lazyInitialState) }]
}
