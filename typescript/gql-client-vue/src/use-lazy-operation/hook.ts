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
    stateRef: ShallowRef<LazyOperationState<OperationResult<TOperation>>>,
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
    stateRef.value = newState
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
    const state =
        shallowRef<LazyOperationState<OperationResult<TOperation>>>(
            lazyInitialState,
        )

    const executeCallback = (
        variables: OperationVariables<TOperation>,
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
