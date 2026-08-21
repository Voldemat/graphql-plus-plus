import {
    isRef,
    onUnmounted,
    shallowRef,
    ShallowRef,
    watch,
    type Ref,
} from 'vue'
import { loadingState } from './loading-state.js'
import type {
    ExecuteResult,
    IExecutor,
    Operation,
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

export type UseOperationHookReturnType<T extends Operation<unknown, unknown>> =
    ShallowRef<OperationState<OperationResult<T>>>
export function useOperation<
    T extends SyncOperation<unknown, unknown>,
    TRequestContext extends RequestContext,
>(
    executor: IExecutor<TRequestContext>,
    operation: T,
    variables: OperationVariables<T> | Ref<OperationVariables<T>>,
    requestContext: TRequestContext | Ref<TRequestContext>,
): UseOperationHookReturnType<T> {
    const state = shallowRef<OperationState<OperationResult<T>>>(loadingState)

    watch(
        [
            () => executor,
            () => operation,
            () => (isRef(variables) ? variables.value : variables),
            () =>
                isRef(requestContext) ? requestContext.value : requestContext,
        ],
        ([exec, op, vars, ctx]) => {
            exec.executeSync(op, vars, ctx)
                .then((result) => {
                    state.value = { state: 'success', ...result }
                })
                .catch((error) => {
                    state.value = { state: 'failure', error: error as Error }
                })
        },
        { immediate: true, deep: true },
    )

    onUnmounted(() => {
        state.value = loadingState
    })

    return state
}
