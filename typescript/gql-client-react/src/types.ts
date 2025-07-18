import { z } from 'zod/v4'

export interface SyncOperation {
    name: string
    document: string;
    type: 'QUERY' | 'MUTATION'
    variablesSchema: z.ZodType;
    resultSchema: z.ZodType;
}

export interface SubscriptionOperation {
    name: string
    document: string;
    type: 'SUBSCRIPTION'
    variablesSchema: z.ZodType;
    resultSchema: z.ZodType;
}

export type Operation = SyncOperation | SubscriptionOperation

export interface ExecuteResult<TResult> {
    result: TResult;
    response: Response;
}

export type OperationResult<T extends Operation> = z.infer<T['resultSchema']>
export type OperationVariables<T extends Operation> =
    z.infer<T['variablesSchema']>
export type SubOpAsyncIterable<T extends Operation> =
    AsyncIterable<OperationResult<T>>

export interface RequestContext {
    fetchOptions?: RequestInit
}

export type Executor<TRequestContext extends RequestContext> = {
    <TSyncOp extends SyncOperation>(
        operation: TSyncOp,
        variables: OperationVariables<TSyncOp>,
        context: TRequestContext
    ): Promise<ExecuteResult<OperationResult<TSyncOp>>>
    <TOperation extends SubscriptionOperation>(
        operation: TOperation,
        variables: OperationVariables<TOperation>,
        context: TRequestContext
    ): Promise<ExecuteResult<SubOpAsyncIterable<TOperation>>>
}
