/* eslint-disable max-lines */
import { getScalarSpecFromMapping, ScalarsMapping } from './scalars/mapping.js';
import ts from 'typescript';
import {
    objectFieldSchema,
    objectFieldSpecSchema,
    objectNonCallableFieldSpecSchema,
    objectSchema,
    objectTypeSchema
} from '@/schema/server.js';
import { z } from 'zod/v4';
import { generateSchemaName, generateZodInferTypeAlias } from './shared.js';
import { invokeMethod } from '../../../shared.js';
import { assertUnreachable } from '../../../../../utils.js';

function generateZodObjectTypeSpec(
    scalarsMapping: ScalarsMapping,
    type: z.infer<typeof objectTypeSchema>
) {
    switch (type._type) {
    case 'Enum': {
        return ts.factory.createCallExpression(
            ts.factory.createPropertyAccessExpression(
                ts.factory.createIdentifier('z'),
                'enum'
            ),
            undefined,
            [ts.factory.createIdentifier(type.name)]
        )
    }
    case 'Scalar': {
        return getScalarSpecFromMapping(scalarsMapping, type.name).outputSchema
    }
    case 'Union':
    case 'InterfaceType':
    case 'ObjectType': {
        return ts.factory.createIdentifier(generateSchemaName(type.name))
    }
    default: {
        assertUnreachable(type)
    }
    }
}

function generateObjectNonCallableFieldSpec(
    scalarsMapping: ScalarsMapping,
    spec: z.infer<typeof objectNonCallableFieldSpecSchema>
) {
    switch (spec._type) {
    case 'array': {
        return ts.factory.createCallExpression(
            ts.factory.createPropertyAccessExpression(
                ts.factory.createIdentifier('z'),
                'array'
            ),
            undefined,
            [generateZodObjectTypeSpec(scalarsMapping, spec.type)]
        )
    }
    case 'literal': {
        return generateZodObjectTypeSpec(scalarsMapping, spec.type)
    }
    default: {
        assertUnreachable(spec)
    }
    }
}

function generateZodObjectFieldSpec(
    scalarsMapping: ScalarsMapping,
    field: z.infer<typeof objectFieldSchema>
) {
    let expression: ts.Expression
    switch (field.spec._type) {
    case 'literal':
    case 'array': {
        expression = generateObjectNonCallableFieldSpec(
            scalarsMapping, field.spec
        )
        break;
    }
    case 'callable': {
        expression = generateObjectNonCallableFieldSpec(
            scalarsMapping, field.spec.returnType
        )
        break;
    }
    default: {
        assertUnreachable(field.spec)
    }
    }
    if (!field.nullable) {
        return expression
    }
    return invokeMethod(
        invokeMethod(expression, 'nullable', []),
        'optional',
        []
    )
}

const lazyTypes: z.infer<typeof objectTypeSchema>['_type'][] = [
    'ObjectType',
    'Union',
    'InterfaceType',
]
function isFieldLazy(spec: z.infer<typeof objectFieldSpecSchema>): boolean {
    switch (spec._type) {
    case 'literal':
    case 'array': return lazyTypes.includes(spec.type._type)
    case 'callable': return lazyTypes.includes(spec.returnType.type._type)
    }
}

function generateObjectTypeDefinitionFields(
    scalarsMapping: ScalarsMapping,
    fields: Record<string, z.infer<typeof objectFieldSchema>>,
    typename?: string
): (ts.PropertyAssignment | ts.GetAccessorDeclaration)[] {
    const assignments = []
    if (typename) {
        assignments.push(ts.factory.createPropertyAssignment(
            '__typename',
            ts.factory.createCallExpression(
                ts.factory.createPropertyAccessExpression(
                    ts.factory.createIdentifier('z'),
                    'literal'
                ),
                undefined,
                [ts.factory.createStringLiteral(typename)]
            )
        ))
    }
    return [
        ...assignments,
        ...Object.entries(fields).map(([name, field]) => {
            const fieldSpec = generateZodObjectFieldSpec(scalarsMapping, field)
            if (isFieldLazy(field.spec)) {
                return ts.factory.createGetAccessorDeclaration(
                    [],
                    name,
                    [],
                    undefined,
                    ts.factory.createBlock([
                        ts.factory.createReturnStatement(fieldSpec)
                    ], true)
                )
            }
            return ts.factory.createPropertyAssignment(
                name,
                fieldSpec,
            )
        })
    ]
}

export function generateZodObjectTypeExpression(
    scalarsMapping: ScalarsMapping,
    object: z.infer<typeof objectSchema>,
    includeTypename: boolean = false
) {
    return ts.factory.createCallExpression(
        ts.factory.createPropertyAccessExpression(
            ts.factory.createIdentifier('z'),
            ts.factory.createIdentifier('object')
        ),
        undefined,
        [ts.factory.createObjectLiteralExpression(
            generateObjectTypeDefinitionFields(
                scalarsMapping,
                object.fields,
                includeTypename ? object.name : undefined
            ), true
        )]
    )
}

export function generateZodObjectTypeNode(
    scalarsMapping: ScalarsMapping,
    object: z.infer<typeof objectSchema>,
) {
    return ts.factory.createVariableStatement(
        [ts.factory.createToken(ts.SyntaxKind.ExportKeyword)],
        ts.factory.createVariableDeclarationList([
            ts.factory.createVariableDeclaration(
                generateSchemaName(object.name),
                undefined,
                undefined,
                generateZodObjectTypeExpression(scalarsMapping, object)
            )
        ],
        ts.NodeFlags.Const),
    )
}

export function generateObjectTypeNodes(
    scalarsMapping: ScalarsMapping,
    object: z.infer<typeof objectSchema>
): ts.Node[] {
    return [
        generateZodObjectTypeNode(scalarsMapping, object),
        generateZodInferTypeAlias(object.name, generateSchemaName(object.name)),
        ts.factory.createIdentifier('\n')
    ]
}
