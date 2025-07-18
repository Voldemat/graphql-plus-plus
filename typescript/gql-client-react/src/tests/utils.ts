import { Operation, SubscriptionOperation } from '@/types.js'
import { z } from 'zod/v4'

export const testOperation = {
    document: '',
    name: '',
    type: 'QUERY',
    variablesSchema: z.object({}),
    resultSchema: z.object({})
} as const satisfies Operation
export type TestOperationResult =
    z.infer<(typeof testOperation)['resultSchema']>

export const testSubscription = {
    document: '',
    name: '',
    type: 'SUBSCRIPTION',
    variablesSchema: z.object({}),
    resultSchema: z.object({ number: z.number() })
} as const satisfies SubscriptionOperation
export type TestSubscriptionResult =
    z.infer<(typeof testSubscription)['resultSchema']>

export async function asyncTimeout(timeoutMS: number) {
    return await new Promise<void>(resolve => setTimeout(resolve, timeoutMS))
}
