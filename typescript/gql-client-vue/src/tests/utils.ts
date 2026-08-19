import { Operation, OperationResult, SubscriptionOperation } from '@/types.js'
import { mount } from '@vue/test-utils'
import { z } from 'zod/v4'

export const testOperation = {
    document: '',
    name: '',
    type: 'QUERY',
    variablesSchema: z.object({}),
    resultSchema: z.object({}),
} as const satisfies Operation<Record<string, never>, Record<string, never>>
export type TestOperationResult = OperationResult<typeof testOperation>

export const testSubscription = {
    document: '',
    name: '',
    type: 'SUBSCRIPTION',
    variablesSchema: z.object({}),
    resultSchema: z.object({ number: z.number() }),
} as const satisfies SubscriptionOperation<
    Record<string, never>,
    { number: number }
>
export type TestSubscriptionResult = OperationResult<typeof testSubscription>

export function mountComposable<T>(composable: () => T) {
    let result: T
    mount({
        setup() {
            result = composable()
            return () => {}
        },
        render() {},
    })
    return result!
}
