import { z } from 'zod/v4'

export interface Operation {
    name: string
    document: string;
    type: 'QUERY' | 'MUTATION' | 'SUBSCRIPTION'
    variablesSchema: z.ZodType;
    resultSchema: z.ZodType;
}

export interface ExecuteResult<T extends Operation> {
    result: z.infer<T['resultSchema']>;
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
    ) => Promise<ExecuteResult<T>>
