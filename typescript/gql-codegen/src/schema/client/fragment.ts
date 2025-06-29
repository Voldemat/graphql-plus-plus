import { z } from 'zod/v4'
import { argument } from './argument.js'

type FragmentSpecSchemaType = z.ZodDiscriminatedUnion<[
    typeof objectFragmentSpec,
    typeof unionFragmentSpec
]>

const fieldSelection =
    z.object({
        _type: z.literal('FieldSelection'),
        name: z.string(),
        alias: z.string(),
        arguments: z.record(z.string(), argument),
        get selection(): z.ZodNullable<FragmentSpecSchemaType> {
            // eslint-disable-next-line no-use-before-define
            return z.nullable(fragmentSpecSchema)
        }
    })

export const typenameSelection = z.object({
    _type: z.literal('TypenameField')
})

type ObjectFragmentSpec = z.ZodObject<{
    _type: z.ZodLiteral<'object'>;
    name: z.ZodString;
    selections: z.ZodArray<typeof objectSelection>
}>
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

type UnionSelection = z.ZodLazy<z.ZodDiscriminatedUnion<[
    typeof typenameSelection,
    typeof spreadSelection,
    typeof objectConditionalSpreadSelection,
    typeof unionConditionalSpreadSelection
]>>

export const unionConditionalSpreadSelection = z.object({
    _type: z.literal('UnionConditionalSpreadSelection'),
    union: z.string(),
    get selections(): z.ZodArray<UnionSelection> {
        // eslint-disable-next-line no-use-before-define
        return z.array(unionSelection)
    }
})

export const unionSelection: UnionSelection =
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
        _type: z.literal('object'),
        name: z.string(),
        selections: z.array(objectSelection)
    })

export const unionFragmentSpec =
    z.object({
        _type: z.literal('union'),
        unionName: z.string(),
        selections: z.array(unionSelection)
    })

export const fragmentSpecSchema: FragmentSpecSchemaType =
    z.discriminatedUnion('_type', [
        objectFragmentSpec,
        unionFragmentSpec
    ])
