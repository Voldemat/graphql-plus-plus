import { describe, it, expect } from 'vitest'
import testJSON from './test.json' with { type: 'json' }
import { rootSchema } from './schema.js'

describe("Schema", () => {
    it('Should parse ok', () => {
        const result = rootSchema.safeParse(testJSON)
        expect(result.success, result.error?.message || '').toBe(true)
    })
})
