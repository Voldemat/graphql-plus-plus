import { useEffect, useMemo, useState } from 'react';
import type {
    IExecutor,
    OperationResult,
    OperationVariables,
    RequestContext,
    SubOpAsyncIterable,
    SubscriptionOperation,
} from './types.js';
import hash, { type NotUndefined } from 'object-hash'
import { type OperationState } from './useOperation.jsx';
import { loadingState } from './loading-state.js';

export function useSubscription<
    T extends SubscriptionOperation<unknown, unknown>,
    TRequestContext extends RequestContext
>(
    executor: IExecutor<TRequestContext>,
    operation: T,
    variables: OperationVariables<T>,
    requestContext: TRequestContext
) {
    const [state, setState] = useState<
        OperationState<SubOpAsyncIterable<OperationResult<T>>>
    >(loadingState)
    const memoizedVariables = useMemo(
        () => variables, [hash(variables as NotUndefined)]
    )
    const memoizedRequestContext = useMemo(
        () => requestContext, [hash(requestContext)]
    )
    useEffect(
        () => {
            executor.executeSubscription(
                operation,
                variables,
                requestContext,
                new AbortController()
            )
                .then(result => setState({ state: 'success', ...result }))
                .catch(error => setState({ state: 'failure', error }))
            return () => {
                setState(currentState => {
                    if (currentState.state === 'success')  {
                        currentState.result.close()
                    }
                    return loadingState
                })
            }
        },
        [
            setState,
            executor,
            operation,
            memoizedVariables,
            memoizedRequestContext
        ]
    )
    return state
}
