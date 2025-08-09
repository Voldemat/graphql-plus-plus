/* eslint-disable max-lines */
import { z } from 'zod/v4';
import { inputFieldSchema } from './shared.js';

export const objectTypeSchema = z.discriminatedUnion('_type', [
    z.object({
        _type: z.literal('ObjectType'),
        name: z.string(),
        $ref: z.string()
    }),
    z.object({
        _type: z.literal('InterfaceType'),
        name: z.string(),
        $ref: z.string()
    }),
    z.object({
        _type: z.literal('Scalar'),
        name: z.string()
    }),
    z.object({
        _type: z.literal('Union'),
        name: z.string(),
        $ref: z.string()
    }),
    z.object({
        _type: z.literal('Enum'),
        name: z.string(),
        $ref: z.string()
    }),
])

export const objectLiteralSpecSchema = z.object({
    _type: z.literal('literal'),
    type: objectTypeSchema
})
export const objectArraySpecSchema = z.object({
    _type: z.literal('array'),
    nullable: z.boolean(),
    type: objectTypeSchema
})

export const objectNonCallableFieldSpecSchema = z.discriminatedUnion('_type', [
    objectLiteralSpecSchema,
    objectArraySpecSchema,
])

export const callableSpecSchema = z.object({
    _type: z.literal('callable'),
    returnType: objectNonCallableFieldSpecSchema,
    arguments: z.record(z.string(), inputFieldSchema)
})
export const objectFieldSpecSchema = z.discriminatedUnion('_type', [
    objectLiteralSpecSchema,
    objectArraySpecSchema,
    callableSpecSchema
])
export const objectFieldSchema = z.object({
    nullable: z.boolean(),
    spec: objectFieldSpecSchema
})
export const objectSchema = z.object({
    name: z.string(),
    implements: z.record(z.string(), z.string()),
    fields: z.record(z.string(), objectFieldSchema),
})

export const inputSchema = z.object({
    name: z.string(),
    fields: z.record(z.string(), inputFieldSchema),
})

export const directiveLocationEnum = z.enum([
    'SCHEMA',
    'SCALAR',
    'OBJECT',
    'FIELD_DEFINITION',
    'ARGUMENT_DEFINITION',
    'INTERFACE',
    'UNION',
    'ENUM',
    'ENUM_VALUE',
    'INPUT_OBJECT',
    'INPUT_FIELD_DEFINITION',
])

export const directiveSchema = z.object({
    name: z.string(),
    locations: z.array(directiveLocationEnum)
})

export const unionSchema = z.object({
    name: z.string(),
    items: z.record(z.string(), z.string()),
})

export const enumSchema = z.object({
    name: z.string(),
    values: z.array(z.string())
})

export const serverSchema = z.object({
    objects: z.record(z.string(), objectSchema),
    directives: z.record(z.string(), directiveSchema),
    unions: z.record(z.string(), unionSchema),
    enums: z.record(z.string(), enumSchema),
    scalars: z.array(z.string()),
    inputs: z.record(z.string(), inputSchema)
})
export type ServerSchema = z.infer<typeof serverSchema>;
