import { z } from 'zod/v4'

export interface SyncOperation {
    type: 'QUERY' | 'MUTATION'
    name: string
    document: string
    variablesSchema: z.ZodType
    resultSchema: z.ZodType
}

export interface SubscriptionOperation {
    type: 'SUBSCRIPTION'
    name: string
    document: string
    variablesSchema: z.ZodType
    resultSchema: z.ZodType
}

export type Operation = SyncOperation | SubscriptionOperation

export interface RequestContext {
    fetchOptions?: RequestInit
}
