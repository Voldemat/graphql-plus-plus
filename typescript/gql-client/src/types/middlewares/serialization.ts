import {
    Operation,
    OperationVariables,
    RequestContext,
    SubscriptionOperation,
    SyncOperation
} from '../base.js'
import { PromiseOrValue } from '../utils.js'

export type BeforeSerializationMiddleware<
    TClientContext,
    TRequestContext extends RequestContext
> = <T extends Operation<unknown, unknown>>(options: {
    clientContext: TClientContext,
    requestContext: TRequestContext,
    operation: T,
    variables: OperationVariables<T>
}) => PromiseOrValue<[T, OperationVariables<T>]>

export type AfterSerializationSyncMiddleware<
    TClientContext,
    TRequestContext extends RequestContext
> = <T extends SyncOperation<unknown, unknown>>(options: {
    clientContext: TClientContext,
    requestContext: TRequestContext,
    operation: T,
    variables: OperationVariables<T>,
    init: RequestInit
}) => PromiseOrValue<RequestInit>

export type AfterSerializationSubscriptionMiddleware<
    TClientContext,
    TRequestContext extends RequestContext
> = <T extends SubscriptionOperation<unknown, unknown>>(options: {
    clientContext: TClientContext,
    requestContext: TRequestContext,
    operation: T,
    variables: OperationVariables<T>,
    init: RequestInit,
    controller: AbortController
}) => PromiseOrValue<RequestInit>
