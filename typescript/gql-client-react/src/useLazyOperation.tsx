import {
    type Dispatch,
    type SetStateAction,
    useCallback,
    useState
} from 'react'
import type {
    Executor,
    OperationResult,
    OperationVariables,
    RequestContext,
    SyncOperation
} from './types.js'
import {
    loadingState,
    type OperationFailureState,
    type OperationState,
    type OperationSuccessState
} from './useOperation.jsx'

export interface LazyOperationInitialState { state: 'initial' }
export type LazyOperationState<TResult> =
    OperationState<TResult> | LazyOperationInitialState
export const lazyInitialState = Object.freeze(
    { state: 'initial' } as const
) satisfies LazyOperationInitialState

type LazyOperationExecuteReturnType<TResult> = Promise<
    OperationSuccessState<TResult> |
    OperationFailureState
>
async function execute<
    TRequestContext extends RequestContext,
    T extends SyncOperation<unknown, unknown>
>(
    executor: Executor<TRequestContext>,
    operation: T,
    variables: OperationVariables<T>,
    requestContext: TRequestContext,
    setState: Dispatch<
        SetStateAction<LazyOperationState<OperationResult<T>>>
    >
): LazyOperationExecuteReturnType<OperationResult<T>> {
    let newState: OperationState<OperationResult<T>>
    try {
        const result = await executor(operation, variables, requestContext)
        newState = { state: 'success', ...result }
    } catch (error: unknown) {
        newState = { state: 'failure', error: error as Error }
    }
    setState(newState)
    return newState
}

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
export function useLazyOperation<
    T extends SyncOperation<unknown, unknown>,
    TRequestContext extends RequestContext
>(
    executor: Executor<TRequestContext>,
    operation: T,
): UseLazyOperationReturnType<
    OperationVariables<T>,
    OperationResult<T>,
    TRequestContext
> {
    const [state, setState] = useState<LazyOperationState<OperationResult<T>>>(
        lazyInitialState
    )
    const executeCallback = useCallback((
        variables: OperationVariables<T>,
        requestContext: TRequestContext
    ) => {
        setState(loadingState)
        return execute(
            executor,
            operation,
            variables,
            requestContext,
            setState
        )
    }, [setState, executor, operation])
    return [
        executeCallback,
        { state, reset: () => setState(lazyInitialState) }
    ]
}
