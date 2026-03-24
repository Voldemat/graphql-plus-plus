import { useSdk } from '@graphql/gql';
import { describe, it, expect } from 'bun:test'

describe('Test getUser', () => {
    const sdk = useSdk();

    it('Test returns ok', async () => {
        const id = '7a703d20-dade-4c67-86bc-eb70aeaf403f';
        const response = await sdk.queries.GetUser({ id }, {})
        expect(response.getUser.id).toBe(id);
        expect(response.getUser.name).toBe("test-name");
        expect(response.getUser.email).toBe("test@gmail.com");
    })
})
