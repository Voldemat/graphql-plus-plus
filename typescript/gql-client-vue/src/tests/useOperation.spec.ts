import { useOperation } from '../useOperation.js'
import { describe, expect, it } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import assert from 'assert'
import { mountComposable, testOperation } from './utils.js'
import type {
    IExecutor,
    OperationResult,
    RequestContext,
    SyncOperation,
} from '@/types.js'

describe('useOperation', () => {
    it('Should return loading state and then success state', async () => {
        const executor = {
            executeSync: async <
                T extends SyncOperation<unknown, unknown>,
            >() => ({
                result: { a: 1 } as OperationResult<T>,
                response: new Response(),
            }),
        } as unknown as IExecutor<RequestContext>

        const state = mountComposable(() =>
            useOperation(executor, testOperation, {}, {}),
        )

        // Synchronously upon invocation, state is 'loading'
        expect(state.value.state).toBe('loading')

        // Wait for async executeSync promise to resolve and flush watchers
        await flushPromises()

        expect(state.value.state).toBe('success')
        assert(state.value.state === 'success')
        expect(state.value.result).toStrictEqual({ a: 1 })
    })

    it('Should return loading state and then failure state', async () => {
        const error = new Error('Network error')
        const executor = {
            executeSync: async () => {
                throw error
            },
        } as unknown as IExecutor<RequestContext>

        const state = mountComposable(() =>
            useOperation(executor, testOperation, {}, {}),
        )

        // Synchronously upon invocation, state is 'loading'
        expect(state.value.state).toBe('loading')

        // Wait for rejected executeSync promise to catch and flush watchers
        await flushPromises()

        expect(state.value.state).toBe('failure')
        assert(state.value.state === 'failure')
        expect(state.value.error).toBe(error)
    })
})
