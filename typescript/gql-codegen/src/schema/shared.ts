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

export const literalSchema = z.union([
    z.string(),
    z.int(),
    z.float32(),
    z.boolean()
])
export const arrayLiteralSchema = z.union([
    z.array(z.string()),
    z.array(z.int()),
    z.array(z.float32()),
    z.array(z.boolean())
])

export const inputLiteralSpecSchema = z.object({
    _type: z.literal('literal'),
    type: inputTypeSchema,
    defaultValue: literalSchema.optional().nullable()
})

export const inputArraySpecSchema = z.object({
    _type: z.literal('array'),
    nullable: z.boolean(),
    type: inputTypeSchema,
    defaultValue: arrayLiteralSchema.optional().nullable()
})

export const inputFieldSpecSchema = z.discriminatedUnion('_type', [
    inputLiteralSpecSchema,
    inputArraySpecSchema,
])

export const inputFieldSchema = z.object({
    nullable: z.boolean(),
    spec: inputFieldSpecSchema
})
