import { useSdk } from '@graphql/gql';
import { describe, it, expect } from 'bun:test'

describe('Test check', () => {
    const sdk = useSdk();

    it('Test returns ok', async () => {
        const response = await sdk.queries.GetCheck({}, {})
        expect(response.getCheck.a).toBe(1)
    })
})
