import {
    MaybeRefOrGetter,
    onUnmounted,
    shallowRef,
    ShallowRef,
    toValue,
    watch,
} from 'vue'
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

export type UseOperationHookReturnType<TResult> = ShallowRef<
    OperationState<TResult>
>
export function useOperation<
    TExecutor extends IExecutor<TRequestContext>,
    TRequestContext extends RequestContext,
    TOperation extends SyncOperation<unknown, unknown>,
>(
    executor: TExecutor,
    operation: TOperation,
    variables: MaybeRefOrGetter<OperationVariables<TOperation>>,
    requestContext: MaybeRefOrGetter<TRequestContext>,
): UseOperationHookReturnType<OperationResult<TOperation>> {
    const state =
        shallowRef<OperationState<OperationResult<TOperation>>>(loadingState)

    watch(
        [() => toValue(variables), () => toValue(requestContext)],
        ([vars, ctx]) => {
            executor
                .executeSync(operation, vars, ctx)
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
