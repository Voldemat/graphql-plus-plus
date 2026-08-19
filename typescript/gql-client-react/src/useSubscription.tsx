import { useEffect, useMemo, useState } from 'react'
import type {
    IExecutor,
    OperationResult,
    OperationVariables,
    RequestContext,
    SubOpAsyncIterable,
    SubscriptionOperation,
} from './types.js'
import hash, { type NotUndefined } from 'object-hash'
import { type OperationState } from './useOperation.jsx'
import { loadingState } from './loading-state.js'

export function useSubscription<
    T extends SubscriptionOperation<unknown, unknown>,
    TRequestContext extends RequestContext,
>(
    executor: IExecutor<TRequestContext>,
    operation: T,
    variables: OperationVariables<T>,
    requestContext: TRequestContext,
) {
    const [state, setState] =
        useState<OperationState<SubOpAsyncIterable<OperationResult<T>>>>(
            loadingState,
        )
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
