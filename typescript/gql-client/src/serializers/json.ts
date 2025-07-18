import { ClientSerializer, RequestContext } from '@/types/index.js'
import assert from 'assert'

export function createJSONSerializer<
    TClientContext,
    TRequestContext extends RequestContext
>(): ClientSerializer<TClientContext, TRequestContext> {
    return {
        serializeRequest: ({ operation, requestContext, variables }) => {
            const headers: Record<string, string> = {
                'Content-Type': 'application/json'
            }
            if (operation.type === 'SUBSCRIPTION') {
                headers.Accept = 'text/event-stream'
            }
            return {
                headers,
                method: 'POST',
                body: JSON.stringify({
                    query: operation.document,
                    variables
                }, (key, value) => {
                    assert(
                        !(value instanceof File),
                        'jsonSerializer cannot encode File objects, ' +
                        `key: "${key}"`
                    )
                    return value
                }),
                ...requestContext.fetchOptions
            }
        },
    }
}
