/* eslint-disable max-lines */
import { z } from 'zod/v4'
import { argument } from './argument.js'

type UnionFragmentSpec = z.ZodObject<{
    _type: z.ZodLiteral<'UnionFragmentSpec'>;
    name: z.ZodString;
    selections: z.ZodArray<typeof unionSelection>
}>
type ObjectFragmentSpec = z.ZodObject<{
    _type: z.ZodLiteral<'ObjectFragmentSpec'>;
    name: z.ZodString;
    selections: z.ZodArray<typeof objectSelection>
}>
export type FragmentSpecSchemaZodType = z.ZodDiscriminatedUnion<[
    ObjectFragmentSpec,
    UnionFragmentSpec
]>
export type FragmentSpecSchemaType =
    {
        _type: 'UnionFragmentSpec',
        name: string,
        selections: UnionSelection[]
    } |
    {
        _type: 'ObjectFragmentSpec',
        name: string,
        selections: z.infer<typeof objectSelection>[]
    }

export const fieldSelection =
    z.object({
        _type: z.literal('FieldSelection'),
        name: z.string(),
        alias: z.string(),
        arguments: z.record(z.string(), argument),
        get selection(): z.ZodNullable<FragmentSpecSchemaZodType> {
            // eslint-disable-next-line no-use-before-define
            return z.nullable(fragmentSpecSchema)
        }
    })

export const typenameSelection = z.object({
    _type: z.literal('TypenameField'),
    alias: z.nullable(z.string())
})

export const objectConditionalSpreadSelection = z.object({
    _type: z.literal('ObjectConditionalSpreadSelection'),
    object: z.string(),
    get spec(): ObjectFragmentSpec {
        // eslint-disable-next-line no-use-before-define
        return objectFragmentSpec
    }
})

export const spreadSelection = z.object({
    _type: z.literal('SpreadSelection'),
    fragment: z.string()
})

export type UnionSelection =
    z.infer<typeof typenameSelection> |
    z.infer<typeof spreadSelection> |
    z.infer<typeof objectConditionalSpreadSelection> |
    z.infer<typeof unionConditionalSpreadSelection>

export type UnionSelectionZodType = z.ZodLazy<
    z.ZodDiscriminatedUnion<[
        typeof typenameSelection,
        typeof spreadSelection,
        typeof objectConditionalSpreadSelection,
        typeof unionConditionalSpreadSelection
    ]>
>


export const unionConditionalSpreadSelection = z.object({
    _type: z.literal('UnionConditionalSpreadSelection'),
    union: z.string(),
    get selections(): z.ZodArray<UnionSelectionZodType> {
        // eslint-disable-next-line no-use-before-define
        return z.array(unionSelection)
    }
})

export const unionSelection: UnionSelectionZodType =
    z.lazy(() => z.discriminatedUnion('_type', [
        typenameSelection,
        spreadSelection,
        objectConditionalSpreadSelection,
        unionConditionalSpreadSelection,
    ]))

export const objectSelection = z.lazy(() => z.discriminatedUnion('_type', [
    fieldSelection,
    typenameSelection,
    spreadSelection
]))

export const objectFragmentSpec: ObjectFragmentSpec =
    z.object({
        _type: z.literal('ObjectFragmentSpec'),
        name: z.string(),
        selections: z.array(objectSelection)
    })

export const unionFragmentSpec =
    z.object({
        _type: z.literal('UnionFragmentSpec'),
        name: z.string(),
        selections: z.array(unionSelection)
    })

export const fragmentSpecSchema =
    z.discriminatedUnion('_type', [
        objectFragmentSpec,
        unionFragmentSpec
    ]) satisfies FragmentSpecSchemaZodType

export const fragmentSchema = z.object({
    sourceText: z.string(),
    spec: fragmentSpecSchema
})
