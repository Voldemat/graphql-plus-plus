import { describe, it, expect } from 'vitest'
import testServerJSON from './test-server.json' with { type: 'json' }
import testClientJSON from './test-client.json' with { type: 'json' }
import { serverSchema } from './server.js'
import { clientSchema } from './client/root.js'

describe('Server schema', () => {
    it('Should parse ok', () => {
        const result = serverSchema.safeParse(testServerJSON)
        expect(
            result.success, result.error?.message || ''
        ).toBe(true)
    })
})

describe('Client schema', () => {
    it('Should parse ok', () => {
        const result = clientSchema.safeParse(testClientJSON)
        expect(
            result.success, result.error?.message || ''
        ).toBe(true)
    })
})
