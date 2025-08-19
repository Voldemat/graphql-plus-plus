import { RequestContext } from '@/types/base.js'
import { ClientSerializer } from '@/types/serializer.js'

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function hasBlobValue(data: any): boolean {
    if (typeof data !== 'object') return false
    for (const value of Object.values(data)) {
        if (typeof value !== 'object' || value === null) continue
        if (value instanceof Blob) return true
        const hasBlob = hasBlobValue(value)
        if (hasBlob) return true
    }
    return false
}

export function createSerializer<
    TClientContext,
    TRequestContext extends RequestContext
>(
    jsonSerializer: ClientSerializer<TClientContext, TRequestContext>,
    multipartSerializer: ClientSerializer<TClientContext, TRequestContext>,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    shouldUseMultipartSerializer: (variables: any) => boolean = hasBlobValue
): ClientSerializer<TClientContext, TRequestContext> {
    return {
        serializeRequest: (options) => {
            if (!shouldUseMultipartSerializer(options.variables)) {
                return jsonSerializer.serializeRequest(options)
            }
            return multipartSerializer.serializeRequest(options)
        },
    }
}
