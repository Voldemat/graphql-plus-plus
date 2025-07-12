import { z } from 'zod/v4'
import { Operation, RequestContext } from './base.js'

export interface ClientParserParseBodyOptions<
    TClientContext,
    TRequestContext extends RequestContext,
    T extends Operation
> {
    clientContext: TClientContext,
    requestContext: TRequestContext
    operation: T,
    response: Response
}

export interface ClientParser<
    TClientContext,
    TRequestContext extends RequestContext
> {
    parseBody: <T extends Operation>(
        options: ClientParserParseBodyOptions<
            TClientContext,
            TRequestContext,
            T
        >
    ) => Promise<z.infer<T['resultSchema']>> | z.infer<T['resultSchema']>
}

