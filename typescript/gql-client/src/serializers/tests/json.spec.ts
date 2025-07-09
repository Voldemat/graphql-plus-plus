import { describe, expect, it } from 'vitest'
import { Operation } from '@/types.js'
import { z } from 'zod/v4'
import { createJSONSerializer } from '../json.js'

describe('Json serializer', () => {
    const jsonSerializer = createJSONSerializer()

    it('Should crush if files are present', async () => {
        const operation = {
            document: 'test-document',
            variablesSchema: z.object({
                name: z.string(),
                file: z.file(),
            }),
            resultSchema: z.void()
        } satisfies Operation
        const variables: z.infer<(typeof operation)['variablesSchema']> = {
            name: 'test-name',
            file: new File([], '')
        }
        expect(
            () => jsonSerializer.serializeRequest({}, operation, variables)
        ).toThrowError('jsonSerializer cannot encode File objects, key: "file"')
    })

    it('Should json stringify variables', async () => {
        const operation = {
            document: 'test-document',
            variablesSchema: z.object({
                name: z.string(),
            }),
            resultSchema: z.void()
        } satisfies Operation
        const variables: z.infer<(typeof operation)['variablesSchema']> = {
            name: 'test-name',
        }
        const init = await jsonSerializer.serializeRequest(
            {},
            operation,
            variables
        )
        const headers = new Headers(init.headers)
        expect(headers.get('Content-Type')).toBe('application/json')
        expect(init.body).toBe(JSON.stringify({
            query: operation.document,
            variables: {
                name: 'test-name',
            }
        }))
    })
})
