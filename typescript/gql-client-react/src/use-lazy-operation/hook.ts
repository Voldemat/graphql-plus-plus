import {
    type Dispatch,
    type SetStateAction,
    useCallback,
    useState
} from 'react'
import type {
    IExecutor,
    OperationResult,
    OperationVariables,
    RequestContext,
    SyncOperation
} from '../types.js'
import { type OperationState } from '../useOperation.jsx'
import {
    LazyOperationExecuteReturnType,
    LazyOperationInitialState,
    LazyOperationState,
    UseLazyOperationReturnType
} from './types.js'
import { loadingState } from '../loading-state.js'

const lazyInitialState = Object.freeze(
    { state: 'initial' } as const
) satisfies LazyOperationInitialState

async function execute<
    TRequestContext extends RequestContext,
    T extends SyncOperation<unknown, unknown>
>(
    executor: IExecutor<TRequestContext>,
    operation: T,
    variables: OperationVariables<T>,
    requestContext: TRequestContext,
    setState: Dispatch<
        SetStateAction<LazyOperationState<OperationResult<T>>>
    >
): LazyOperationExecuteReturnType<OperationResult<T>> {
    let newState: OperationState<OperationResult<T>>
    try {
        const result = await executor.executeSync(
            operation,
            variables,
            requestContext
        )
        newState = { state: 'success', ...result }
    } catch (error: unknown) {
        newState = { state: 'failure', error: error as Error }
    }
    setState(newState)
    return newState
}

export function useLazyOperation<
    T extends SyncOperation<unknown, unknown>,
    TRequestContext extends RequestContext
>(
    executor: IExecutor<TRequestContext>,
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
