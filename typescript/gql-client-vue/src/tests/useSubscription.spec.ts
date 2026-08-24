import { useSubscription } from '../useSubscription.js'
import { describe, expect, it } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import assert from 'assert'
import { mountComposable, testSubscription } from './utils.js'
import type { IExecutor, RequestContext } from '@/types.js'

describe('useSubscription', () => {
    it('Should return loading state and then success state', async () => {
        const executor = {
            executeSubscription: async () => {
                const readableStream = new ReadableStream()
                const response = new Response(readableStream)
                return {
                    result: {
                        stream: (async function* () {
                            yield { number: 1 }
                            yield { number: 2 }
                            yield { number: 3 }
                        })(),
                        close: () => {},
                    },
                    response,
                }
            },
        } as unknown as IExecutor<RequestContext>

        const state = mountComposable(() =>
            useSubscription(executor, testSubscription, {}, {}),
        )

        // Synchronously upon invocation, state is 'loading'
        expect(state.value.state).toBe('loading')

        // Wait for async executeSubscription promise to resolve and flush watchers
        await flushPromises()

        expect(state.value.state).toBe('success')
        assert(state.value.state === 'success')

        let number = 1
        for await (const value of state.value.result.stream) {
            expect(value.number).toBe(number)
            number++
        }
    })

    it('Should return loading state and then failure state', async () => {
        const error = new Error('Network error')
        const executor = {
            executeSubscription: async () => {
                throw error
            },
        } as unknown as IExecutor<RequestContext>

        const state = mountComposable(() =>
            useSubscription(executor, testSubscription, {}, {}),
        )

        // Synchronously upon invocation, state is 'loading'
        expect(state.value.state).toBe('loading')

        // Wait for rejected executeSubscription promise to catch and flush watchers
        await flushPromises()

        expect(state.value.state).toBe('failure')
        assert(state.value.state === 'failure')
        expect(state.value.error).toBe(error)
    })
})
