import { ClientSerializer } from '../types.js';

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function hasBlobValue (data: any): boolean  {
    if (typeof data !== 'object') return false
    for (const value of Object.values(data)) {
        if (typeof value !== 'object' || value === null) continue
        if (value instanceof Blob) return true
        const hasBlob = hasBlobValue(value)
        if (hasBlob) return true
    }
    return false
}

export function createSerializer<TContext>(
    jsonSerializer: ClientSerializer<TContext>,
    multipartSerializer: ClientSerializer<TContext>,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    shouldUseMultipartSerializer: (variables: any) => boolean = hasBlobValue
): ClientSerializer<TContext> {
    return {
        serializeRequest: (context, operation, variables) => {
            if (!shouldUseMultipartSerializer(variables)) {
                return jsonSerializer.serializeRequest(
                    context, operation, variables
                )
            }
            return multipartSerializer.serializeRequest(
                context, operation, variables
            )
        },
    }
}
