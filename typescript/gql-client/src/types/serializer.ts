import { Operation, RequestContext } from './base.js'
import { OperationVariables, PromiseOrValue } from './utils.js'

export interface ClientSerializer<
    TClientContext,
    TRequestContext extends RequestContext
> {
    serializeRequest: <T extends Operation>(options: {
        clientContext: TClientContext,
        requestContext: TRequestContext,
        operation: T,
        variables: OperationVariables<T>
    }) => PromiseOrValue<RequestInit>
}

