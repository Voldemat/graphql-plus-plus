import { z } from 'zod/v4'
import { Operation, RequestContext } from './base.js'

export interface ClientSerializer<
    TClientContext,
    TRequestContext extends RequestContext
> {
    serializeRequest: <T extends Operation>(options: {
        clientContext: TClientContext,
        requestContext: TRequestContext,
        operation: T,
        variables: z.infer<T['variablesSchema']>
    }) => Promise<RequestInit> | RequestInit
}

