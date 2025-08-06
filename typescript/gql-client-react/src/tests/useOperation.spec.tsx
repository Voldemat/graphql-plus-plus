import { useOperation } from '../useOperation.jsx';
import { describe, expect, it } from 'vitest'
import { renderHook, act } from '@testing-library/react'
import assert from 'assert';
import { testOperation } from './utils.js';
import {
    IExecutor,
    OperationResult,
    RequestContext,
    SyncOperation
} from '@/types.js';

describe('useOperation', () => {
    it('Should return loading state and then success state', async () => {
        const executor = {
            executeSync: async <
                T extends SyncOperation<unknown, unknown>
            >() => ({
                result: { a: 1 } as OperationResult<T>,
                response: new Response()
            }),
        } as unknown as IExecutor<RequestContext>
        const { result } = renderHook((props) => useOperation(...props), {
            initialProps: [executor, testOperation, {}, {}] as const
        })
        expect(result.current.state).toBe('loading')
        act(() => { })
        await act(async () => { })
        expect(result.current.state).toBe('success')
        assert(result.current.state === 'success')
        expect(result.current.result).toStrictEqual({ a: 1 })
    })

    it('Should return loading state and then failure state', async () => {
        const error = new Error('Network error')
        const executor = {
            executeSync: async() => { throw error }
        } as unknown as IExecutor<RequestContext>
        const { result } = renderHook((props) => useOperation(...props), {
            initialProps: [executor, testOperation, {}, {}] as const
        })
        expect(result.current.state).toBe('loading')
        act(() => { })
        await act(async () => { })
        expect(result.current.state).toBe('failure')
        assert(result.current.state === 'failure')
        expect(result.current.error).toBe(error)
    })
})
