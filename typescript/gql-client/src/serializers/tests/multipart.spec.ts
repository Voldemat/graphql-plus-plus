import { describe, expect, it } from 'vitest'
import { createMultipartSerializer } from '../multipart.js'
import { Operation, OperationVariables } from '@/types/index.js'
import { z } from 'zod/v4'
import assert from 'assert'

describe('Multipart serializer', () => {
    const multipartSerializer = createMultipartSerializer()

    it('Should crush if no files present', () => {
        const operation = {
            document: 'test-document',
            name: '',
            type: 'QUERY',
            variablesSchema: z.object({
                name: z.string()
            }),
            resultSchema: z.void(),
        } satisfies Operation<{ name: string }, void>
        const variables: OperationVariables<typeof operation> = {
            name: 'test-name'
        }
        expect(
            () => multipartSerializer.serializeRequest({
                clientContext: {},
                requestContext: {},
                operation,
                variables
            })
        ).toThrowError('Dont use multipartSerializer for regular bodies')
    })

    it('Should build proper form data', async () => {
        const operation = {
            document: 'test-document',
            name: '',
            type: 'QUERY',
            variablesSchema: z.object({
                name: z.string(),
                file: z.file()
            }),
            resultSchema: z.void(),
        } satisfies Operation<{ name: string, file: File }, void>
        const variables: OperationVariables<typeof operation> = {
            name: 'test-name',
            file: new File([], 'check.txt')
        }
        const init = await multipartSerializer.serializeRequest({
            clientContext: {},
            requestContext: {},
            operation,
            variables
        })
        const headers = new Headers(init.headers)
        expect(headers.get('Content-Type')).toBe(null)
        assert(init.body != null)
        assert(init.body instanceof FormData)
        expect(init.body.get('operations')).toBe(JSON.stringify({
            query: operation.document,
            variables: {
                name: 'test-name',
                file: null
            }
        }))
        expect(init.body.get('map')).toBe(JSON.stringify({
            '0': ['variables.file']
        }))
        expect(init.body.get('0')).toStrictEqual(variables.file)
    })
})
