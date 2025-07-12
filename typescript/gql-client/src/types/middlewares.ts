import { z } from 'zod/v4'
import { Operation, RequestContext } from './base.js'

export type BeforeSerializationMiddleware<
    TClientContext,
    TRequestContext extends RequestContext
> = <T extends Operation>(options: {
    clientContext: TClientContext,
    requestContext: TRequestContext,
    operation: T,
    variables: z.infer<T['variablesSchema']>
}) => Promise<[T, z.infer<T['variablesSchema']>]> |
    [T, z.infer<T['variablesSchema']>]

export type AfterSerializationMiddleware<
    TClientContext,
    TRequestContext extends RequestContext
> = <T extends Operation>(options: {
    clientContext: TClientContext,
    requestContext: TRequestContext,
    operation: T,
    variables: z.infer<T['variablesSchema']>,
    init: RequestInit
}) => Promise<RequestInit> | RequestInit

export type BeforeParsingMiddleware<
    TClientContext,
    TRequestContext extends RequestContext
> = <T extends Operation>(options: {
    clientContext: TClientContext,
    requestContext: TRequestContext,
    operation: T,
    variables: z.infer<T['variablesSchema']>,
    init: RequestInit,
    response: Response
}) => Promise<Response> | Response

export type AfterParsingMiddleware<
    TClientContext,
    TRequestContext extends RequestContext
> = <T extends Operation>(options: {
    clientContext: TClientContext,
    requestContext: TRequestContext,
    operation: T,
    variables: z.infer<T['variablesSchema']>,
    init: RequestInit,
    response: Response,
    result: z.infer<T['resultSchema']>
}) => Promise<z.infer<T['resultSchema']>> |
    z.infer<T['resultSchema']>

export interface ClientMiddlewaresConfig<
    TClientContext,
    TRequestContext extends RequestContext
> {
    beforeSerialization: BeforeSerializationMiddleware<
        TClientContext, TRequestContext
    >[]
    afterSerialization: AfterSerializationMiddleware<
        TClientContext, TRequestContext
    >[]
    beforeParsing: BeforeParsingMiddleware<TClientContext, TRequestContext>[]
    afterParsing: AfterParsingMiddleware<TClientContext, TRequestContext>[]
}
