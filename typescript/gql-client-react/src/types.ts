import { z } from 'zod/v4'

export interface Operation {
    name: string
    document: string;
    type: 'QUERY' | 'MUTATION' | 'SUBSCRIPTION'
    variablesSchema: z.ZodType;
    resultSchema: z.ZodType;
}

export interface ExecuteResult<TResult> {
    result: TResult;
    response: Response;
}

export interface RequestContext {
    fetchOptions?: RequestInit
}

export type Executor<TRequestContext extends RequestContext> =
    <T extends Operation>(
        operation: T,
        variables: z.infer<T['variablesSchema']>,
        context: TRequestContext
    ) => Promise<ExecuteResult<z.infer<T['resultSchema']>>>
