import { z } from 'zod/v4'
import { Executor, Operation, RequestContext } from './types.js'
import {
    loadingState,
    type OperationFailureState,
    type OperationState,
    type OperationSuccessState
} from './useOperation.jsx'
import { Dispatch, SetStateAction, useState } from 'react'

export interface LazyOperationInitialState {
    state: 'initial'
}
export type LazyOperationState<TResult> =
    OperationState<TResult> | LazyOperationInitialState
export const lazyInitialState = Object.freeze(
    { state: 'initial' } as const
) satisfies LazyOperationInitialState

type LazyOperationExecuteReturnType<TResult> = Promise<
    OperationSuccessState<TResult> |
    OperationFailureState
>
async function execute<
    TRequestContext extends RequestContext,
    T extends Operation
>(
    executor: Executor<TRequestContext>,
    operation: T,
    variables: z.infer<T['variablesSchema']>,
    requestContext: TRequestContext,
    setState: Dispatch<
        SetStateAction<LazyOperationState<z.infer<T['resultSchema']>>>
    >
): LazyOperationExecuteReturnType<z.infer<T['resultSchema']>> {
    let newState: OperationState<z.infer<T['resultSchema']>>
    try {
        const result = await executor(operation, variables, requestContext)
        newState = { state: 'success', ...result }
    } catch (error: unknown) {
        newState = { state: 'failure', error: error as Error }
    }
    setState(newState)
    return newState
}

export interface UseLazyOperationReturnType<
    TVariables,
    TResult,
    TRequestContext extends RequestContext
> {
    execute: (
        variables: TVariables,
        requestContext: TRequestContext
    ) => LazyOperationExecuteReturnType<TResult>
    state: LazyOperationState<TResult>
    reset: () => void
}
export function useLazyOperation<
    T extends Operation,
    TRequestContext extends RequestContext
>(
    executor: Executor<TRequestContext>,
    operation: T,
): UseLazyOperationReturnType<
    z.infer<T['variablesSchema']>,
    z.infer<T['resultSchema']>,
    TRequestContext
> {
    const [state, setState] = useState<
        LazyOperationState<z.infer<T['resultSchema']>>
    >(lazyInitialState)
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
