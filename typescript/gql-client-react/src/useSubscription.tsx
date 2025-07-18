import { useEffect, useMemo, useState } from 'react';
import type {
    Executor,
    OperationVariables,
    RequestContext,
    SubOpAsyncIterable,
    SubscriptionOperation,
} from './types.js';
import hash, { type NotUndefined } from 'object-hash'
import { loadingState, type OperationState } from './useOperation.jsx';

export function useSubscription<
    T extends SubscriptionOperation,
    TRequestContext extends RequestContext
>(
    executor: Executor<TRequestContext>,
    operation: T,
    variables: OperationVariables<T>,
    requestContext: TRequestContext
): OperationState<SubOpAsyncIterable<T>> {
    const [state, setState] = useState<OperationState<SubOpAsyncIterable<T>>>(
        loadingState
    )
    const memoizedVariables = useMemo(
        () => variables, [hash(variables as NotUndefined)]
    )
    const memoizedRequestContext = useMemo(
        () => requestContext, [hash(requestContext)]
    )
    useEffect(
        () => {
            executor(operation, variables, requestContext)
                .then(result => setState({ state: 'success', ...result }))
                .catch(error => setState({ state: 'failure', error }))
            return () => setState(loadingState)
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
