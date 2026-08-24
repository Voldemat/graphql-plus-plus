import {
    MaybeRefOrGetter,
    onUnmounted,
    shallowRef,
    toValue,
    watch,
    type ShallowRef,
} from 'vue'
import { loadingState } from './loading-state.js'
import type {
    IExecutor,
    OperationResult,
    OperationVariables,
    RequestContext,
    SubOpAsyncIterable,
    SubscriptionOperation,
} from './types.js'
import { type OperationState } from './useOperation.js'

export type UseSubscriptionHookReturnType<TResult> = ShallowRef<
    OperationState<SubOpAsyncIterable<TResult>>
>

export function useSubscription<
    TExecutor extends IExecutor<TRequestContext>,
    TRequestContext extends RequestContext,
    TOperation extends SubscriptionOperation<unknown, unknown>,
>(
    executor: TExecutor,
    operation: TOperation,
    variables: MaybeRefOrGetter<OperationVariables<TOperation>>,
    requestContext: MaybeRefOrGetter<TRequestContext>,
): UseSubscriptionHookReturnType<OperationResult<TOperation>> {
    const state =
        shallowRef<
            OperationState<SubOpAsyncIterable<OperationResult<TOperation>>>
        >(loadingState)

    watch(
        [() => toValue(variables), () => toValue(requestContext)],
        ([vars, ctx], _, onCleanup) => {
            const controller = new AbortController()
            onCleanup(() => {
                if (state.value.state === 'success') {
                    state.value.result.close()
                } else {
                    controller.abort()
                }
                state.value = loadingState
            })
            executor
                .executeSubscription(operation, vars, ctx, controller)
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
