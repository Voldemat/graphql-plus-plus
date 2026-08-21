import { shallowRef, type ShallowRef } from 'vue'
import { loadingState } from '../loading-state.js'
import type {
    IExecutor,
    OperationResult,
    OperationVariables,
    RequestContext,
    SyncOperation,
} from '../types.js'
import { type OperationState } from '../useOperation.js'
import type {
    UseLazyOperationHookReturnType,
    LazyOperationExecuteReturnType,
    LazyOperationInitialState,
    LazyOperationState,
} from './types.js'

const lazyInitialState = Object.freeze({
    state: 'initial',
} as const) satisfies LazyOperationInitialState

async function execute<
    TRequestContext extends RequestContext,
    T extends SyncOperation<unknown, unknown>,
>(
    executor: IExecutor<TRequestContext>,
    operation: T,
    variables: OperationVariables<T>,
    requestContext: TRequestContext,
    stateRef: ShallowRef<LazyOperationState<OperationResult<T>>>,
): LazyOperationExecuteReturnType<OperationResult<T>> {
    let newState: OperationState<OperationResult<T>>
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
    stateRef.value = newState
    return newState
}

export function useLazyOperation<
    TRequestContext extends RequestContext,
    T extends SyncOperation<unknown, unknown>,
>(
    executor: IExecutor<TRequestContext>,
    operation: T,
): UseLazyOperationHookReturnType<
    TRequestContext,
    OperationVariables<T>,
    OperationResult<T>
> {
    const state =
        shallowRef<LazyOperationState<OperationResult<T>>>(lazyInitialState)

    const executeCallback = (
        variables: OperationVariables<T>,
        requestContext: TRequestContext,
    ) => {
        state.value = loadingState
        return execute(executor, operation, variables, requestContext, state)
    }

    const reset = () => {
        state.value = lazyInitialState
    }

    return [executeCallback, { state, reset }]
}
