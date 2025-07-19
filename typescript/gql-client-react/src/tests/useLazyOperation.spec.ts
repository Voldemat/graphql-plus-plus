import { z } from 'zod/v4'
import { LazyOperationState, useLazyOperation } from '../useLazyOperation.jsx';
import { describe, expect, it } from 'vitest'
import { renderHook, act } from '@testing-library/react'
import assert from 'assert';
import { testOperation, TestOperationResult } from './utils.js';
import { Executor, Operation, RequestContext } from '@/types.js';

describe('useLazyOperation', () => {
    it('Should preserve initial state if no one calls', async () => {
        const { result } = renderHook(() =>
            useLazyOperation({} as Executor<RequestContext>, testOperation))
        expect(result.current[1].state.state).toBe('initial')
        await act(async () => {})
        expect(result.current[1].state.state).toBe('initial')
    })

    it('Should return loading state after invoking, then success state',
        async () => {
            const executor: Executor<RequestContext> =
                async <T extends Operation<unknown, unknown>>() => ({
                    result: { a: 1 } as z.infer<T['resultSchema']>,
                    response: new Response()
                })
            const { result } = renderHook(() =>
                useLazyOperation(executor, testOperation))
            let promise: Promise<LazyOperationState<TestOperationResult>>
            act(() => {
                promise = result.current[0]({}, {})
            })
            expect(result.current[1].state.state).toBe('loading')
            // @ts-expect-error 2454
            const newState = await promise
            expect(newState.state).toBe('success')
            assert(newState.state === 'success')
            expect(newState.result).toStrictEqual({ a: 1 })
            await act(async () => {})
            expect(result.current[1].state).toStrictEqual(newState)
        })

    it('Should return loading state after invoking, then failure state',
        async () => {
            const error = new Error('Network error')
            const executor: Executor<RequestContext> =
                async () => { throw error }
            const { result } = renderHook(() =>
                useLazyOperation(executor, testOperation))
            let promise: Promise<LazyOperationState<TestOperationResult>>
            act(() => {
                promise = result.current[0]({}, {})
            })
            expect(result.current[1].state.state).toBe('loading')
            // @ts-expect-error 2454
            const newState = await promise 
            expect(newState.state).toBe('failure')
            assert(newState.state === 'failure')
            expect(newState.error).toBe(error)
            await act(async () => {})
            expect(result.current[1].state).toStrictEqual(newState)
        })
})
