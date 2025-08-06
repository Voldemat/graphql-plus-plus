import { RequestContext } from './base.js'
import { ClientMiddlewaresConfig } from './middlewares/config.js'
import { ClientParser } from './parser.js'
import { ClientSerializer } from './serializer.js'

export interface ClientConfig<
    TClientContext,
    TRequestContext extends RequestContext
> {
    context: TClientContext
    parser: ClientParser<TClientContext, TRequestContext>
    serializer: ClientSerializer<TClientContext, TRequestContext>
    middlewares: ClientMiddlewaresConfig<TClientContext, TRequestContext>
    fetcher: (init: RequestInit) => Promise<Response>
}
