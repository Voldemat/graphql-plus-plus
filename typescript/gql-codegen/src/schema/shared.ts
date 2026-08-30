import { z } from 'zod/v4';

export const inputTypeSchema = z.discriminatedUnion('_type', [
    z.object({
        _type: z.literal('InputType'),
        name: z.string(),
        $ref: z.string(),
    }),
    z.object({
        _type: z.literal('Scalar'),
        name: z.string(),
    }),
    z.object({
        _type: z.literal('Enum'),
        name: z.string(),
        $ref: z.string(),
    }),
]);

export const literalSchema = z.discriminatedUnion('_type', [
    z.object({
        _type: z.literal('string'),
        value: z.string(),
    }),
    z.object({
        _type: z.literal('enum-value'),
        value: z.string(),
    }),
    z.object({
        _type: z.literal('null'),
    }),
    z.object({
        _type: z.literal('int'),
        value: z.number(),
    }),
    z.object({
        _type: z.literal('float'),
        value: z.number(),
    }),
    z.object({
        _type: z.literal('boolean'),
        value: z.boolean(),
    }),
]);

export const arrayLiteralSchema = z.union([
    z.array(z.string()),
    z.array(z.int()),
    z.array(z.float32()),
    z.array(z.boolean()),
]);

export const inputLiteralSpecSchema = z.object({
    _type: z.literal('literal'),
    type: inputTypeSchema,
    defaultValue: literalSchema.optional(),
});

export const inputArraySpecSchema = z.object({
    _type: z.literal('array'),
    nullable: z.boolean(),
    defaultValue: arrayLiteralSchema.optional(),
    get type(): z.ZodDiscriminatedUnion<
        [typeof inputLiteralSpecSchema, typeof inputArraySpecSchema]
    > {
        // eslint-disable-next-line no-use-before-define
        return inputFieldSpecSchema;
    },
});

export const inputFieldSpecSchema = z.discriminatedUnion('_type', [
    inputLiteralSpecSchema,
    inputArraySpecSchema,
]);

export const inputFieldSchema = z.object({
    nullable: z.boolean(),
    spec: inputFieldSpecSchema,
});
