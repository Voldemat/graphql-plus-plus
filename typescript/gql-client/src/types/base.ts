import { z } from 'zod/v4'

export interface Operation {
    type: 'QUERY' | 'MUTATION' | 'SUBSCRIPTION'
    name: string
    document: string
    variablesSchema: z.ZodType
    resultSchema: z.ZodType
}

export interface RequestContext {
    fetchOptions?: RequestInit
}

