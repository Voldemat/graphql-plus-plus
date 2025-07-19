import { z } from 'zod/v4'
import { useOperation } from '../useOperation.jsx';
import { describe, expect, it } from 'vitest'
import { renderHook, act } from '@testing-library/react'
import assert from 'assert';
import { testOperation } from './utils.js';
import { Executor, Operation, RequestContext } from '@/types.js';

describe('useOperation', () => {
    it('Should return loading state and then success state', async () => {
        const executor: Executor<RequestContext> =
            async <T extends Operation<unknown, unknown>>() => ({
                result: { a: 1 } as z.infer<T['resultSchema']>,
                response: new Response()
            })
        const { result } = renderHook((props) => useOperation(...props), {
            initialProps: [executor, testOperation, {}, {}] as const
        })
        expect(result.current.state).toBe('loading')
        act(() => {})
        await act(async () => {})
        expect(result.current.state).toBe('success')
        assert(result.current.state === 'success')
        expect(result.current.result).toStrictEqual({ a: 1 })
    })

    it('Should return loading state and then failure state', async () => {
        const error = new Error('Network error')
        const executor: Executor<RequestContext> = async () => { throw error }
        const { result } = renderHook((props) => useOperation(...props), {
            initialProps: [executor, testOperation, {}, {}] as const
        })
        expect(result.current.state).toBe('loading')
        act(() => {})
        await act(async () => {})
        expect(result.current.state).toBe('failure')
        assert(result.current.state === 'failure')
        expect(result.current.error).toBe(error)
    })
})
