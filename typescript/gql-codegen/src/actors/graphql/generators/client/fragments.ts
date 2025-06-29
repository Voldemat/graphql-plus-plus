/* eslint-disable max-lines */
import ts from 'typescript';
import {
    createQuestionTokenIfNullable,
    createTypenamePropertySignature,
    generateScalarReference,
} from '../shared.js';
import {
    fieldSelection,
    FragmentSpecSchemaType,
    objectConditionalSpreadSelection,
    objectSelection,
    unionSelection
} from '../../../../schema/client/fragment.js';
import { z } from 'zod/v4';
import { RootSchema } from '../../../../schema/root.js';
import assert from 'assert';
import {
    objectFieldSpecSchema,
    objectTypeSchema
} from '../../../../schema/server.js';
import { wrapInMaybeIfNullable } from '../server/shared.js';

function generateFragmentUnionSpecTypeNodes(
    scalars: string[],
    schema: RootSchema,
    spec: Extract<FragmentSpecSchemaType, { _type: 'union' }>
): ts.TypeNode[] {
    const objectSelections =
        // eslint-disable-next-line no-use-before-define
        getObjectConditionalSpreadSelectionsFromUnionSelections(
            schema,
            spec.selections
        )
    return objectSelections.map(s => {
        return ts.factory.createTypeLiteralNode([
            createTypenamePropertySignature(s.object),
            // eslint-disable-next-line no-use-before-define
            ...generateFragmentObjectSpecPropertySignatures(
                scalars,
                schema,
                s.spec,
                true
            )
        ])
    })
}

function generateFieldLiteralSelectionTypeNode(
    scalars: string[],
    schema: RootSchema,
    typeSpec: z.infer<typeof objectTypeSchema>,
    selection: FragmentSpecSchemaType | null
) {
    if (typeSpec._type === 'Scalar') {
        assert(selection === null)
        return generateScalarReference(typeSpec.name)
    }
    assert(selection !== null)
    if (selection._type === 'object') {
        return ts.factory.createTypeLiteralNode(
            // eslint-disable-next-line no-use-before-define
            generateFragmentObjectSpecPropertySignatures(
                scalars,
                schema,
                selection,
                false
            )
        )
    }
    return ts.factory.createUnionTypeNode(
        generateFragmentUnionSpecTypeNodes(scalars, schema, selection)
    )
}

function generateFieldSelectionSpecTypeNode(
    scalars: string[],
    schema: RootSchema,
    fieldSpec: z.infer<typeof objectFieldSpecSchema>,
    selection: FragmentSpecSchemaType | null
): ts.TypeNode {
    switch (fieldSpec._type) {
    case 'literal': {
        return generateFieldLiteralSelectionTypeNode(
            scalars,
            schema,
            fieldSpec.type,
            selection
        )
    }
    case 'array': {
        const elementType = generateFieldLiteralSelectionTypeNode(
            scalars,
            schema,
            fieldSpec.type,
            selection
        )
        return ts.factory.createArrayTypeNode(
            fieldSpec.nullable ?
                ts.factory.createUnionTypeNode([
                    elementType,
                    ts.factory.createLiteralTypeNode(ts.factory.createNull())
                ]) :
                elementType

        )
    }
    case 'callable': {
        return generateFieldSelectionSpecTypeNode(
            scalars, schema, fieldSpec.returnType, selection
        )
    }
    }
}

function generateFieldSelectionPropertySignature(
    scalars: string[],
    schema: RootSchema,
    typeName: string,
    selection: z.infer<typeof fieldSelection>
): ts.PropertySignature {
    const type = schema.server.objects[typeName]
    const field = type.fields[selection.name]
    return ts.factory.createPropertySignature(
        undefined,
        selection.alias,
        createQuestionTokenIfNullable(field.nullable),
        wrapInMaybeIfNullable(generateFieldSelectionSpecTypeNode(
            scalars,
            schema,
            field.spec,
            selection.selection as FragmentSpecSchemaType | null
        ), field.nullable)
    )
}

function generateObjectSelectionPropertySignatures(
    scalars: string[],
    schema: RootSchema,
    typeName: string,
    selection: z.infer<typeof objectSelection>,
): ts.PropertySignature[] {
    switch (selection._type) {
    case 'TypenameField': {
        return [createTypenamePropertySignature(typeName)]
    }
    case 'SpreadSelection': {
        const fragment = schema.client.fragments[
            selection.fragment
        ] as FragmentSpecSchemaType
        assert(fragment._type === 'object')
        // eslint-disable-next-line no-use-before-define
        return generateFragmentObjectSpecPropertySignatures(
            scalars,
            schema,
            fragment,
            true
        )
    }
    case 'FieldSelection': {
        return [generateFieldSelectionPropertySignature(
            scalars,
            schema,
            typeName,
            selection
        )]
    }
    }
}

function generateFragmentObjectSpecPropertySignatures(
    scalars: string[],
    schema: RootSchema,
    spec: Extract<FragmentSpecSchemaType, { _type: 'object' }>,
    skipTypename: boolean
): ts.PropertySignature[] {
    const nodes = spec.selections.map(selection => {
        return generateObjectSelectionPropertySignatures(
            scalars,
            schema,
            spec.name,
            selection,
        )
    }).flat()
    if (!skipTypename) {
        return [
            createTypenamePropertySignature(spec.name),
            ...nodes
        ]
    }
    return nodes
}

function getObjectConditionalSpreadSelectionsFromUnionSelections(
    schema: RootSchema,
    selections: z.infer<typeof unionSelection>[]
): z.infer<typeof objectConditionalSpreadSelection>[] {
    const objectSelections:
        Record<string, z.infer<typeof objectConditionalSpreadSelection>> = {}
    for (const selection of selections) {
        if (
            selection._type === 'TypenameField' ||
            selection._type === 'UnionConditionalSpreadSelection'
        ) continue
        if (selection._type === 'ObjectConditionalSpreadSelection') {
            if (!(selection.object in objectSelections)) {
                objectSelections[selection.object] = selection
            }
            continue
        }
        const fragment = schema.client.fragments[
            selection.fragment
        ] as FragmentSpecSchemaType
        assert(fragment._type === 'union')
        for (const s of getObjectConditionalSpreadSelectionsFromUnionSelections(
            schema,
            fragment.selections
        )) {
            if (!(s.object in objectSelections)) {
                objectSelections[s.object] = s
            }
        }
    }
    return Object.values(objectSelections)
}

function generateUnionFragmentChildren(
    scalars: string[],
    schema: RootSchema,
    unionName: string,
    selections: z.infer<typeof objectConditionalSpreadSelection>[],
): ts.Node[] {
    return selections.map(s => {
        // eslint-disable-next-line no-use-before-define
        return generateObjectFragmentSpecDeclaration(
            scalars, schema, `${unionName}_${s.object}_Fragment`, s.spec
        )
    }).flat()
}

function generateObjectFragmentSpecDeclaration(
    scalars: string[],
    schema: RootSchema,
    fragmentName: string,
    spec: Extract<FragmentSpecSchemaType, { _type: 'object' }>
) {
    return ts.factory.createInterfaceDeclaration(
        ts.factory.createModifiersFromModifierFlags(
            ts.ModifierFlags.Export
        ),
        fragmentName,
        undefined,
        undefined,
        generateFragmentObjectSpecPropertySignatures(
            scalars,
            schema,
            spec,
            false
        )
    )
}

function generateFragmentSpecDeclarations(
    scalars: string[],
    schema: RootSchema,
    name: string,
    spec: FragmentSpecSchemaType
): ts.Node[] {
    const fragmentName = name + 'Fragment'
    if (spec._type === 'object') {
        return [generateObjectFragmentSpecDeclaration(
            scalars,
            schema,
            fragmentName,
            spec
        )]
    }
    const objectSelections =
        getObjectConditionalSpreadSelectionsFromUnionSelections(
            schema,
            spec.selections
        )
    return [
        ...generateUnionFragmentChildren(
            scalars, schema, name, objectSelections
        ),
        ts.factory.createTypeAliasDeclaration(
            ts.factory.createModifiersFromModifierFlags(
                ts.ModifierFlags.Export
            ),
            fragmentName,
            undefined,
            ts.factory.createUnionTypeNode(
                objectSelections.map(s =>
                    ts.factory.createTypeReferenceNode(
                        `${name}_${s.object}_Fragment`
                    ))
            )
        )
    ]
}

export function generateFragmentTypes(
    scalars: string[],
    schema: RootSchema,
): ts.Node[] {
    return Object.entries(
        schema.client.fragments as Record<string, FragmentSpecSchemaType>
    ).map(([name, spec]) => {
        return generateFragmentSpecDeclarations(scalars, schema, name, spec)
    }).flat()
}
