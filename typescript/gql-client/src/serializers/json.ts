import { ClientSerializer } from '@/types.js'
import assert from 'assert'

export function createJSONSerializer<TContext>(): ClientSerializer<TContext> {
    return {
        serializeRequest: (_, operation, variables) => {
            return {
                headers: {
                    'Content-Type': 'application/json'
                },
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
                })
            }
        },
    }
}
