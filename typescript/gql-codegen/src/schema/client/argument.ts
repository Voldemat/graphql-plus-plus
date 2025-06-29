import { z } from 'zod/v4';

export const argumentValue = z.discriminatedUnion('_type', [
    z.object({
        _type: z.literal('ref'),
        name: z.string()
    }),
    z.object({
        _type: z.literal('literal'),
        value: z.union([z.string(), z.number(), z.boolean()])
    })
])

export const argument = z.object({
    name: z.string(),
    value: argumentValue,
})
