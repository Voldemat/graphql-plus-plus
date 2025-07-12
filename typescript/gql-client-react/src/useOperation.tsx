import { useEffect, useState } from 'react';
import { z } from 'zod/v4'
import { ExecuteResult, Executor, Operation, RequestContext } from './types.js';

export interface OperationLoadingState {
    state: 'loading'
}

export interface OperationSuccessState<
    T extends Operation
> extends ExecuteResult<T> {
    state: 'success'
}

export interface OperationFailureState {
    state: 'failure'
    error: Error
}

export type OperationState<T extends Operation> =
    OperationLoadingState | OperationSuccessState<T> | OperationFailureState

export const loadingState =
    Object.freeze({ state: 'loading' } as const) satisfies OperationLoadingState

export function useOperation<
    T extends Operation,
    TRequestContext extends RequestContext
>(
    executor: Executor<TRequestContext>,
    operation: T,
    variables: z.infer<T['variablesSchema']>,
    requestContext: TRequestContext
): OperationState<T> {
    const [state, setState] = useState<OperationState<T>>(loadingState)
    useEffect(() => {
        executor(operation, variables, requestContext)
            .then(result => setState({ state: 'success', ...result }))
            .catch(error => setState({ state: 'failure', error }))
        return () => setState(loadingState)
    }, [])
    return state
}
