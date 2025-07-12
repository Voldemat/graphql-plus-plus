import { createContext, ReactNode, useContext } from 'react'
import { Executor, RequestContext } from './types.js'

export function createProviderAndHook<TRequestContext extends RequestContext>(
) {
    const context = createContext<Executor<TRequestContext>>(
        undefined as unknown as Executor<TRequestContext>
    )
    function Provider({
        children,
        executor
    }: { children: ReactNode, executor: Executor<TRequestContext> }) {
        return (
            <context.Provider value={executor}>
                {children}
            </context.Provider>
        )
    }
    function useExecutor(): Executor<TRequestContext> {
        return useContext(context)
    }
    return {
        useGQLExecutor: useExecutor,
        GQLExecutorProvider: Provider
    }
}


