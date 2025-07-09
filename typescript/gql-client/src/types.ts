import { z } from 'zod/v4'

export interface Operation {
    document: string
    variablesSchema: z.ZodType
    resultSchema: z.ZodType
}

export interface ClientParser<TContext> {
    parseBody: <T extends Operation>(
        context: TContext,
        operation: T,
        response: Response
    ) => Promise<z.infer<T['resultSchema']>> |
    z.infer<T['resultSchema']>
}

export interface ClientSerializer<TContext> {
    serializeRequest: <T extends Operation>(
        context: TContext,
        operation: T,
        variables: z.infer<T['variablesSchema']>
    ) => Promise<RequestInit> | RequestInit
}

export type BeforeSerializationMiddleware<TContext> = <T extends Operation>(
    context: TContext,
    operation: T,
    variables: z.infer<T['variablesSchema']>
) => Promise<[T, z.infer<T['variablesSchema']>]> |
    [T, z.infer<T['variablesSchema']>]

export type AfterSerializationMiddleware<TContext> = <T extends Operation>(
    context: TContext,
    operation: T,
    variables: z.infer<T['variablesSchema']>,
    init: RequestInit
) => Promise<RequestInit> | RequestInit

export type BeforeParsingMiddleware<TContext> = <T extends Operation>(
    context: TContext,
    operation: T,
    variables: z.infer<T['variablesSchema']>,
    init: RequestInit,
    response: Response
) => Promise<Response> | Response

export type AfterParsingMiddleware<TContext> = <T extends Operation>(
    context: TContext,
    operation: T,
    variables: z.infer<T['variablesSchema']>,
    init: RequestInit,
    response: Response,
    result: z.infer<T['resultSchema']>
) => Promise<z.infer<T['resultSchema']>> |
    z.infer<T['resultSchema']>

export interface ClientMiddlewaresConfig<TContext> {
    beforeSerialization: BeforeSerializationMiddleware<TContext>[]
    afterSerialization: AfterSerializationMiddleware<TContext>[]
    beforeParsing: BeforeParsingMiddleware<TContext>[]
    afterParsing: AfterParsingMiddleware<TContext>[]
}

export interface ClientConfig<TContext> {
    parser: ClientParser<TContext>
    serializer: ClientSerializer<TContext>
    middlewares: ClientMiddlewaresConfig<TContext>
    context: TContext
    fetcher: (init: RequestInit) => Promise<Response>
}
