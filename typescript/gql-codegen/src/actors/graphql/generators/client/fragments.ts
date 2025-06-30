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

export function generateFragmentUnionSpecTypeNodes(
    scalars: string[],
    schema: RootSchema,
    spec: Extract<FragmentSpecSchemaType, { _type: 'union' }>,
): ts.TypeNode[] {
    const union = schema.server.unions[spec.unionName]
    const typenameSelections =[
        ...spec.selections.filter(s => s._type === 'TypenameField'),
        
    ]
    if (typenameSelections.length === 0 ||
        typenameSelections.every(tSelection => tSelection.alias !== null)) {
        typenameSelections.push({ _type: 'TypenameField', alias: null })
    }
    const globalHasTypenameAliasedField = typenameSelections
        .some(tSelection => tSelection.alias !== null)
    const selectedTypes: string[] = []
    const selectedNodes =
    spec.selections.filter(s => s._type !== 'TypenameField').map(s => {
        assert(s._type !== 'UnionConditionalSpreadSelection')
        const nodes: ts.PropertySignature[] = []
        if (s._type === 'ObjectConditionalSpreadSelection') {
            selectedTypes.push(s.object)
            const hasTypenameAliasedField = s.spec.selections
                .some(o => o._type === 'TypenameField' && o.alias !== null)
            nodes.push(
                ...typenameSelections.map(
                    tSelection =>
                        createTypenamePropertySignature(
                            s.object,
                            hasTypenameAliasedField,
                            tSelection.alias
                        )
                ),
                // eslint-disable-next-line no-use-before-define
                ...generateFragmentObjectSpecPropertySignatures(
                    scalars,
                    schema,
                    s.spec,
                    { ensurePresent: true, optional: false, ignore: true }
                )
            )
        }
        return ts.factory.createTypeLiteralNode(nodes)
    })
    const unselectedTypes = Object.keys(union.items)
        .filter(item => !selectedTypes.includes(item))
    return [
        ...selectedNodes,
        ...unselectedTypes.map(typeName =>
            ts.factory.createTypeLiteralNode(
                typenameSelections.map(
                    tSelection =>
                        createTypenamePropertySignature(
                            typeName,
                            tSelection.alias === null &&
                                globalHasTypenameAliasedField,
                            tSelection.alias
                        )
                )
            ))
    ]
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
                { ensurePresent: true, optional: true, ignore: false }
            )
        )
    }
    const unionNodes = generateFragmentUnionSpecTypeNodes(
        scalars,
        schema,
        selection,
    )
    return ts.factory.createUnionTypeNode(unionNodes)
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
    assert(type)
    const field = type.fields[selection.name]
    assert(field, JSON.stringify({ fields: type.fields, name: selection.name }))
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
    typenameOptional: boolean
): ts.PropertySignature[] {
    switch (selection._type) {
    case 'TypenameField': {
        return [
            createTypenamePropertySignature(
                typeName,
                !typenameOptional || selection.alias === null,
                selection.alias
            )
        ]
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
            { ensurePresent: false, optional: true, ignore: false }
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

export function generateFragmentObjectSpecPropertySignatures(
    scalars: string[],
    schema: RootSchema,
    spec: Extract<FragmentSpecSchemaType, { _type: 'object' }>,
    typenameConfig: {
        ensurePresent: boolean,
        optional: boolean,
        ignore: boolean
    }
): ts.PropertySignature[] {
    let hasTypename: boolean = false
    const nodes = spec.selections.map(selection => {
        if (selection._type === 'TypenameField' && typenameConfig.ignore)
            return []
        if (selection._type === 'TypenameField') hasTypename = true
        return generateObjectSelectionPropertySignatures(
            scalars,
            schema,
            spec.name,
            selection,
            typenameConfig.optional
        )
    }).flat()
    if (
        !typenameConfig.ignore &&
        !hasTypename &&
        typenameConfig.ensurePresent
    ) {
        return [
            createTypenamePropertySignature(
                spec.name,
                typenameConfig.optional,
                null
            ),
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
            { ensurePresent: true, optional: true, ignore: false }
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
        return [
            generateObjectFragmentSpecDeclaration(
                scalars,
                schema,
                fragmentName,
                spec
            )
        ]
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
