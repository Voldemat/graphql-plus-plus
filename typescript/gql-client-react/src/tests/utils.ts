import { Operation } from '@/types.js'
import { z } from 'zod/v4'

export const testOperation = {
    document: '',
    name: '',
    type: 'QUERY',
    variablesSchema: z.object({}),
    resultSchema: z.object({})
} as const satisfies Operation
