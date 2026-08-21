import {
    isRef,
    onUnmounted,
    shallowRef,
    watch,
    type Ref,
    type ShallowRef,
} from 'vue'
import { loadingState } from './loading-state.js'
import type {
    IExecutor,
    Operation,
    OperationResult,
    OperationVariables,
    RequestContext,
    SubOpAsyncIterable,
    SubscriptionOperation,
} from './types.js'
import { type OperationState } from './useOperation.js'

export type UseSubscriptionHookReturnType<
    T extends Operation<unknown, unknown>,
> = ShallowRef<OperationState<SubOpAsyncIterable<OperationResult<T>>>>

export function useSubscription<
    T extends SubscriptionOperation<unknown, unknown>,
    TRequestContext extends RequestContext,
>(
    executor: IExecutor<TRequestContext>,
    operation: T,
    variables: OperationVariables<T> | Ref<OperationVariables<T>>,
    requestContext: TRequestContext | Ref<TRequestContext>,
): UseSubscriptionHookReturnType<T> {
    const state =
        shallowRef<OperationState<SubOpAsyncIterable<OperationResult<T>>>>(
            loadingState,
        )

    watch(
        [
            () => executor,
            () => operation,
            () => (isRef(variables) ? variables.value : variables),
            () =>
                isRef(requestContext) ? requestContext.value : requestContext,
        ],
        ([exec, op, vars, ctx], _oldValues, onCleanup) => {
            const controller = new AbortController()
            onCleanup(() => {
                if (state.value.state === 'success') {
                    state.value.result.close()
                } else {
                    controller.abort()
                }
                state.value = loadingState
            })
            exec.executeSubscription(op, vars, ctx, controller)
                .then((result) => {
                    if (controller.signal.aborted) return
                    state.value = { state: 'success', ...result }
                })
                .catch((error) => {
                    if (controller.signal.aborted) return
                    state.value = {
                        state: 'failure',
                        error: error as Error,
                    }
                })
        },
        { immediate: true, deep: true },
    )

    onUnmounted(() => {
        if (state.value.state === 'success') {
            state.value.result.close()
        }
        state.value = loadingState
    })

    return state
}
