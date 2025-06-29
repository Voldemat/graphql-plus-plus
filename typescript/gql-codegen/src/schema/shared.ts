import { z } from 'zod/v4';

export const inputTypeSchema = z.discriminatedUnion('_type', [
    z.object({
        _type: z.literal('InputType'),
        name: z.string(),
        $ref: z.string()
    }),
    z.object({
        _type: z.literal('Scalar'),
        name: z.string()
    }),
    z.object({
        _type: z.literal('Enum'),
        name: z.string(),
        $ref: z.string()
    }),
])

export const inputLiteralSpecSchema = z.object({
    _type: z.literal('literal'),
    type: inputTypeSchema
})
export const inputArraySpecSchema = z.object({
    _type: z.literal('array'),
    nullable: z.boolean(),
    type: inputTypeSchema
})
export const inputFieldSpecSchema = z.discriminatedUnion('_type', [
    inputLiteralSpecSchema,
    inputArraySpecSchema,
])

export const inputFieldSchema = z.object({
    nullable: z.boolean(),
    spec: inputFieldSpecSchema
})
