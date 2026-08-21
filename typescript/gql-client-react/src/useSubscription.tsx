import hash, { type NotUndefined } from 'object-hash'
import { useEffect, useMemo, useState } from 'react'
import { loadingState } from './loading-state.js'
import type {
    IExecutor,
    OperationResult,
    OperationVariables,
    RequestContext,
    SubOpAsyncIterable,
    SubscriptionOperation,
} from './types.js'
import { type OperationState } from './useOperation.jsx'

export type UseSubscriptionHookReturnType<TResult> = OperationState<
    SubOpAsyncIterable<TResult>
>

export function useSubscription<
    TExecutor extends IExecutor<TRequestContext>,
    TRequestContext extends RequestContext,
    TOperation extends SubscriptionOperation<unknown, unknown>,
>(
    executor: TExecutor,
    operation: TOperation,
    variables: OperationVariables<TOperation>,
    requestContext: TRequestContext,
): UseSubscriptionHookReturnType<OperationResult<TOperation>> {
    const [state, setState] =
        useState<
            OperationState<SubOpAsyncIterable<OperationResult<TOperation>>>
        >(loadingState)
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
        const controller = new AbortController()
        executor
            .executeSubscription(
                operation,
                memoizedVariables,
                memoizedRequestContext,
                controller,
            )
            .then((result) => {
                if (controller.signal.aborted) return
                setState({ state: 'success', ...result })
            })
            .catch((error) => {
                if (controller.signal.aborted) return
                setState({ state: 'failure', error })
            })
        return () => {
            setState((currentState) => {
                if (currentState.state === 'success') {
                    currentState.result.close()
                } else {
                    controller.abort()
                }
                return loadingState
            })
        }
    }, [
        setState,
        executor,
        operation,
        memoizedVariables,
        memoizedRequestContext,
    ])
    return state
}
