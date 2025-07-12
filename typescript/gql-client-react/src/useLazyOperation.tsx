import { z } from 'zod/v4'
import { Executor, Operation, RequestContext } from './types.js'
import { loadingState, OperationState } from './useOperation.jsx'
import { Dispatch, SetStateAction, useState } from 'react'

export interface LazyOperationInitialState {
    state: 'initial'
}
export type LazyOperationState<T extends Operation> =
    OperationState<T> | LazyOperationInitialState
export const lazyInitialState = Object.freeze(
    { state: 'initial' } as const
) satisfies LazyOperationInitialState

async function execute<
    TRequestContext extends RequestContext,
    T extends Operation
>(
    executor: Executor<TRequestContext>,
    operation: T,
    variables: z.infer<T['variablesSchema']>,
    requestContext: TRequestContext,
    setState: Dispatch<SetStateAction<LazyOperationState<T>>>
): Promise<LazyOperationState<T>> {
    let newState: OperationState<T>
    try {
        const result = await executor(operation, variables, requestContext)
        newState = { state: 'success', ...result }
    } catch (error: unknown) {
        newState = { state: 'failure', error: error as Error }
    }
    setState(newState)
    return newState
}

interface UseLazyOperationReturnType<
    T extends Operation,
    TRequestContext extends RequestContext
> {
    execute: (
        variables: z.infer<T['variablesSchema']>,
        requestContext: TRequestContext
    ) => Promise<LazyOperationState<T>>
    state: LazyOperationState<T>
    reset: () => void
}
export function useLazyOperation<
    T extends Operation,
    TRequestContext extends RequestContext
>(
    executor: Executor<TRequestContext>,
    operation: T,
): UseLazyOperationReturnType<T, TRequestContext> {
    const [state, setState] = useState<LazyOperationState<T>>(lazyInitialState)
    return {
        execute: (variables, requestContext) => {
            setState(loadingState)
            return execute(
                executor,
                operation,
                variables,
                requestContext,
                setState
            )
        },
        state,
        reset: () => setState(lazyInitialState)
    }
}
