import type { StreamUsersResult } from '@graphql/generated/graphql';
import { useSdk } from '@graphql/gql';
import { awaitWithTimeout } from '@shared/utils';
import { describe, it, expect } from 'bun:test'

describe('Test streamUsers', () => {
    const sdk = useSdk();

    it('Test returns ok', async () => {
        const controller = new AbortController()
        const response = await sdk.subscriptions.StreamUsers({}, {}, controller)
        await awaitWithTimeout<void>((async () => {
            const stream: AsyncIterable<StreamUsersResult, void, unknown> =
                response.stream
            const iterator = stream[Symbol.asyncIterator]();
            const first = await iterator.next()
            expect(first.done).toBe(false)
            expect(first.value).not.toBeNil()
            const second = await iterator.next()
            expect(second.done).toBe(false)
            expect(second.value).not.toBeNil()
            const third = await iterator.next()
            expect(third.done).toBe(true)
        })(), 1000);
    })
})
