import { useSubscription } from '../useSubscription.jsx';
import { describe, expect, it } from 'vitest'
import { renderHook, act } from '@testing-library/react'
import assert from 'assert';
import { testSubscription } from './utils.js';
import { Executor, RequestContext } from '@/types.js';

describe('useSubscription', () => {
    it('Should return loading state and then success state', async () => {
        const executor: Executor<RequestContext> = async () => {
            const readableStream = new ReadableStream()
            const response = new Response(readableStream)
            return {
                result: {
                    stream: (async function*() {
                        yield { number: 1 }
                        yield { number: 2 }
                        yield { number: 3 }
                    })(),
                    close: () => {}
                },
                response
            }
        }
        const { result } = renderHook((props) => useSubscription(...props), {
            initialProps: [executor, testSubscription, {}, {}] as const
        })
        expect(result.current.state).toBe('loading')
        act(() => { })
        await act(async () => { })
        expect(result.current.state).toBe('success')
        assert(result.current.state === 'success')
        let number = 1
        for await (const value of result.current.result.stream) {
            expect(value.number).toBe(number)
            number++
        }
    })

    it('Should return loading state and then failure state', async () => {
        const error = new Error('Network error')
        const executor: Executor<RequestContext> = async () => { throw error }
        const { result } = renderHook((props) => useSubscription(...props), {
            initialProps: [executor, testSubscription, {}, {}] as const
        })
        expect(result.current.state).toBe('loading')
        act(() => { })
        await act(async () => { })
        expect(result.current.state).toBe('failure')
        assert(result.current.state === 'failure')
        expect(result.current.error).toBe(error)
    })
})
