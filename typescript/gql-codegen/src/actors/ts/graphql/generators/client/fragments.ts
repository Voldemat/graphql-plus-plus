/* eslint-disable max-lines */
import ts from 'typescript';
import { RootSchema } from '@/schema/root.js';
import {
    fragmentSchema,
    FragmentSpecSchemaType,
    objectConditionalSpreadSelection,
    objectSelection,
    typenameSelection,
    unionFragmentSpec,
    UnionSelection,
} from '@/schema/client/fragment.js';
import { z } from 'zod/v4';
import { objectSchema } from '@/schema/server.js';
import { generateSchemaName } from '../server/shared.js';
import { ScalarsMapping } from '../server/scalars/mapping.js';
import { generateZodObjectFieldSpec } from '../server/objects.js';
import assert from 'assert';
import { invokeMethod } from '../../../shared.js';

export function extractFragmentSourceTextsInSpec(
    schema: RootSchema,
    fragmentSpec: FragmentSpecSchemaType
): string[] {
    if (fragmentSpec._type === 'union') {
        return fragmentSpec.selections.map((s): string[] => {
            if (s._type === 'SpreadSelection') {
                return [schema.client.fragments[s.fragment].sourceText]
            }
            if (s._type === 'ObjectConditionalSpreadSelection') {
                return extractFragmentSourceTextsInSpec(schema, s.spec)
            }
            return []
        }).flat()
    }
    return fragmentSpec.selections.map((s): string[] => {
        if (s._type === 'SpreadSelection') {
            return [schema.client.fragments[s.fragment].sourceText]
        }
        if (s._type === 'FieldSelection' && s.selection !== null) {
            return extractFragmentSourceTextsInSpec(
                schema,
                s.selection as FragmentSpecSchemaType
            )
        }
        return []
    }).flat()
}

function generateFragmentDocumentNode(
    schema: RootSchema,
    name: string,
    fragment: z.infer<typeof fragmentSchema>
) {
    return ts.factory.createVariableStatement(
        [ts.factory.createToken(ts.SyntaxKind.ExportKeyword)],
        ts.factory.createVariableDeclarationList(
            [ts.factory.createVariableDeclaration(
                ts.factory.createIdentifier(name + 'FragmentDocument'),
                undefined,
                undefined,
                ts.factory.createStringLiteral([
                    fragment.sourceText,
                    ...extractFragmentSourceTextsInSpec(
                        schema,
                        fragment.spec as FragmentSpecSchemaType
                    )
                ].join('\n'))
            )],
            ts.NodeFlags.Const
        )
    )
}

function generateZodObjectSelection(
    scalarsMapping: ScalarsMapping,
    schema: RootSchema,
    objectType: z.infer<typeof objectSchema>,
    selection: z.infer<typeof objectSelection>,
    typenameConfig: Parameters<typeof resolveSelections>[1]
): ts.PropertyAssignment | ts.SpreadAssignment | null {
    switch (selection._type) {
    case 'TypenameField': {
        if ('ignore' in typenameConfig) return null
        let expression = invokeMethod(
            ts.factory.createIdentifier('z'),
            'literal',
            [ts.factory.createStringLiteral(objectType.name)]
        )
        if (selection.alias === null && typenameConfig.optional) {
            expression = invokeMethod(
                invokeMethod(expression, 'nullable', []),
                'optional',
                []
            )
        }
        return ts.factory.createPropertyAssignment(
            selection.alias || '__typename',
            expression
        )
    }
    case 'FieldSelection': {
        const fieldSpec = objectType.fields[selection.name]
        let expression: ts.Expression
        if (selection.selection === null) {
            expression = generateZodObjectFieldSpec(scalarsMapping, fieldSpec)
        } else {
            // eslint-disable-next-line no-use-before-define
            expression = generateZodFragmentSpecCallExpression(
                scalarsMapping,
                schema,
                selection.selection,
                { ensurePresent: true, optional: !fieldSpec.nullable }
            )
        }
        if (fieldSpec.nullable) {
            expression = invokeMethod(
                invokeMethod(expression, 'nullable', []),
                'optional',
                []
            )
        }
        return ts.factory.createPropertyAssignment(selection.alias, expression)
    }
    case 'SpreadSelection': {
        return ts.factory.createSpreadAssignment(
            ts.factory.createPropertyAccessExpression(
                ts.factory.createIdentifier(
                    generateSchemaName(selection.fragment + 'Fragment')
                ),
                'shape'
            )
        )
    }
    }
}

function resolveSelections(
    specSelections: z.infer<typeof objectSelection>[],
    typenameConfig:
        { ensurePresent: boolean, optional: boolean } | { ignore: true }
) {
    const ignoreTypename = 'ignore' in typenameConfig
    const selections = [...specSelections.filter(
        s => s._type !== 'TypenameField' || !ignoreTypename
    ).toSorted((s1, s2) => s1._type.localeCompare(s2._type))]
    if (ignoreTypename) return selections
    const hasTypename = specSelections.some(
        s => s._type === 'TypenameField'
    )
    const hasSpreadSelection = specSelections.some(
        s => s._type === 'SpreadSelection'
    )
    if (!hasTypename) {
        if (hasSpreadSelection) {
            selections.push({ _type: 'TypenameField', alias: null })
        } else {
            selections.unshift({ _type: 'TypenameField', alias: null })
        }
    }
    return selections
}


function generateZodObjectFragmentSpecCallExpression(
    scalarsMapping: ScalarsMapping,
    schema: RootSchema,
    object: z.infer<typeof objectSchema>,
    specSelections: z.infer<typeof objectSelection>[],
    insideLazy: boolean,
    typenameConfig: Parameters<typeof resolveSelections>[1]
) {
    const selections = resolveSelections(specSelections, typenameConfig)
    const expression = ts.factory.createCallExpression(
        ts.factory.createPropertyAccessExpression(
            ts.factory.createIdentifier('z'),
            'object'
        ),
        undefined,
        [ts.factory.createObjectLiteralExpression(
            selections.map(s => generateZodObjectSelection(
                scalarsMapping,
                schema,
                object,
                s,
                typenameConfig
            )).filter(s => s !== null),
            true
        )]
    )
    if (insideLazy) return expression
    const hasSpreadSelection = selections.some(
        s => s._type === 'SpreadSelection'
    )
    if (!hasSpreadSelection) return expression
    return ts.factory.createCallExpression(
        ts.factory.createPropertyAccessExpression(
            ts.factory.createIdentifier('z'),
            'lazy'
        ),
        undefined,
        [ts.factory.createArrowFunction(
            undefined,
            undefined,
            [],
            undefined,
            ts.factory.createToken(ts.SyntaxKind.EqualsGreaterThanToken),
            expression
        )]
    )
}

function resolveUnionSelections(
    schema: RootSchema,
    specSelections: UnionSelection[]
): [
    z.infer<typeof objectConditionalSpreadSelection>[],
    z.infer<typeof typenameSelection>[]
] {
    const typenameSelections: z.infer<typeof typenameSelection>[] = []
    const objectSelections = specSelections.map(s => {
        assert(s._type !== 'UnionConditionalSpreadSelection')
        if (s._type === 'TypenameField') {
            typenameSelections.push(s)
            return []
        }
        if (s._type === 'ObjectConditionalSpreadSelection') return [s]
        const fragmentSpec = schema.client.fragments[s.fragment].spec
        assert(fragmentSpec._type === 'union')
        const [selections, tSelections] = resolveUnionSelections(
            schema,
            fragmentSpec.selections
        )
        typenameSelections.push(...tSelections)
        return selections
    }).flat()
    return [objectSelections, typenameSelections]
}

function generateZodUnionFragmentSpecCallExpression(
    scalarsMapping: ScalarsMapping,
    schema: RootSchema,
    spec: z.infer<typeof unionFragmentSpec>
) {
    const [objectSelections, typenameSelections] = resolveUnionSelections(
        schema,
        spec.selections
    )
    for (const item of Object.keys(
        schema.server.unions[spec.unionName].items
    )) {
        if (!objectSelections.some(s => s.object === item)) {
            objectSelections.push({
                _type: 'ObjectConditionalSpreadSelection',
                object: item,
                spec: {
                    _type: 'object',
                    name: item,
                    selections: []
                }
            })
        }
    }
    const expression = ts.factory.createCallExpression(
        ts.factory.createPropertyAccessExpression(
            ts.factory.createIdentifier('z'),
            'union'
        ),
        undefined,
        [ts.factory.createArrayLiteralExpression(
            objectSelections.map(s =>
                generateZodObjectFragmentSpecCallExpression(
                    scalarsMapping,
                    schema,
                    schema.server.objects[s.object],
                    [...s.spec.selections, ...typenameSelections],
                    true,
                    { ensurePresent: true, optional: false }
                )),
            true
        )]
    )
    return ts.factory.createCallExpression(
        ts.factory.createPropertyAccessExpression(
            ts.factory.createIdentifier('z'),
            'lazy'
        ),
        undefined,
        [ts.factory.createArrowFunction(
            undefined,
            undefined,
            [],
            undefined,
            ts.factory.createToken(ts.SyntaxKind.EqualsGreaterThanToken),
            expression
        )]
    )
}

export function generateZodFragmentSpecCallExpression(
    scalarsMapping: ScalarsMapping,
    schema: RootSchema,
    spec: FragmentSpecSchemaType,
    typenameConfig?: Parameters<typeof resolveSelections>[1]
) {
    if (spec._type === 'object') {
        return generateZodObjectFragmentSpecCallExpression(
            scalarsMapping,
            schema,
            schema.server.objects[spec.name],
            spec.selections,
            false,
            typenameConfig || { ensurePresent: true, optional: true }
        )
    }
    return generateZodUnionFragmentSpecCallExpression(
        scalarsMapping,
        schema,
        spec
    )

}

function generateZodFragmentSchema(
    scalarsMapping: ScalarsMapping,
    schema: RootSchema,
    fragmentName: string,
    fragment: z.infer<typeof fragmentSchema>
) {
    return ts.factory.createVariableStatement(
        [ts.factory.createToken(ts.SyntaxKind.ExportKeyword)],
        ts.factory.createVariableDeclarationList(
            [ts.factory.createVariableDeclaration(
                ts.factory.createIdentifier(
                    generateSchemaName(fragmentName + 'Fragment')
                ),
                undefined,
                undefined,
                generateZodFragmentSpecCallExpression(
                    scalarsMapping,
                    schema,
                    fragment.spec
                )
            )],
            ts.NodeFlags.Const
        )
    )
}

function generateFragmentSpecDeclarations(
    scalarsMapping: ScalarsMapping,
    schema: RootSchema,
    fragmentName: string,
    fragment: z.infer<typeof fragmentSchema>
): ts.Node[] {
    return [
        generateFragmentDocumentNode(schema, fragmentName, fragment),
        generateZodFragmentSchema(
            scalarsMapping,
            schema,
            fragmentName,
            fragment
        )
    ]
}


export function generateFragmentTypes(
    scalarsMapping: ScalarsMapping,
    schema: RootSchema,
): ts.Node[] {
    return Object.entries(schema.client.fragments)
        .map(([name, fragment]) => {
            return generateFragmentSpecDeclarations(
                scalarsMapping,
                schema,
                name,
                fragment
            )
        }).flat()
}
