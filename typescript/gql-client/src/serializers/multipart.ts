/* eslint-disable @typescript-eslint/no-explicit-any,max-lines */
import { ClientSerializer, Operation, RequestContext } from '@/types/index.js'
import assert from 'assert';

function getFilesKeysAndPayload(
    variables: Record<string, any> | Array<any>,
    initialPath: string = '',
    shouldTreatAsObject?: (v: any) => boolean,
): [string[], Record<string, any> | Array<any>] {
    const filesKeys: string[] = []
    const newObject: any = Array.isArray(variables) ? [] : {}
    for (const [key, value] of Object.entries(variables)) {
        if (value instanceof Blob) {
            filesKeys.push(initialPath + key);
            newObject[key] = null
            continue
        }
        if (typeof value === 'object' && value !== null && (
            shouldTreatAsObject === undefined ||
            shouldTreatAsObject(value)
        )) {
            const [valueFilesKeys, valueObj] = getFilesKeysAndPayload(
                value, initialPath + key + '.', shouldTreatAsObject
            )
            filesKeys.push(...valueFilesKeys);
            newObject[key] = valueObj
            continue
        }
        newObject[key] = value
    }

    return [filesKeys, newObject]
}

function getNestedValue(obj: Record<string, any>, key: string): any {
    const subkeys = key.split('.')
    while (subkeys.length > 0) {
        const currentKey = subkeys.shift()
        if (currentKey === undefined) throw new Error()
        obj = obj[currentKey]
    }
    return obj
}

function buildFormData<T extends Operation<unknown, unknown>>(
    operation: T,
    variables: Record<string, any> | Array<any>,
    shouldTreatAsObject?: (v: any) => boolean
): FormData {
    const formData = new FormData()
    const [filesKeys, finalVariables] = getFilesKeysAndPayload(
        variables, '', shouldTreatAsObject
    )
    assert(
        filesKeys.length !== 0,
        'Dont use multipartSerializer for regular bodies'
    )
    formData.append(
        'operations',
        JSON.stringify({
            query: operation.document,
            variables: finalVariables
        })
    )
    const properties: Array<[string, string]> = filesKeys
        .map((key, index) => [String(index), key])
    const finalMap = Object.fromEntries(
        properties
            .map(([index, key]) => {
                return [index, ['variables.' + key]]
            })
    )
    formData.append('map', JSON.stringify(finalMap))
    for (const [index, key] of properties) {
        const value: Blob | File = getNestedValue(variables, key)
        let filename: string | undefined = undefined
        if (value instanceof File) filename = value.name
        formData.append(index, value, filename)
    }
    return formData
}

export function createMultipartSerializer<
    TClientContext,
    TRequestContext extends RequestContext
>(): ClientSerializer<TClientContext, TRequestContext> {
    return {
        serializeRequest: ({ operation, requestContext, variables }) => {
            const headers: Record<string, string> = {
            }
            if (operation.type === 'SUBSCRIPTION') {
                headers.Accept = 'text/event-stream'
            }
            return {
                method: 'POST',
                headers,
                body: buildFormData(
                    operation,
                    operation.variablesSchema.parse(variables) as any
                ),
                ...requestContext.fetchOptions
            }
        },
    }
}
