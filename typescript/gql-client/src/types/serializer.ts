import { Operation, OperationVariables, RequestContext } from './base.js'
import { PromiseOrValue } from './utils.js'

export interface ClientSerializer<
    TClientContext,
    TRequestContext extends RequestContext
> {
    serializeRequest: <T extends Operation<unknown, unknown>>(options: {
        clientContext: TClientContext,
        requestContext: TRequestContext,
        operation: T,
        variables: OperationVariables<T>
    }) => PromiseOrValue<RequestInit>
}
