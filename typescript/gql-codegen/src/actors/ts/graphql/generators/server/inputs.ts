/* eslint-disable max-lines */
import { z } from 'zod/v4';
import ts from 'typescript';
import { inputSchema } from '@/schema/server.js';
import {
    inputFieldSchema,
    inputFieldSpecSchema,
    inputTypeSchema
} from '@/schema/shared.js';
import { getScalarSpecFromMapping, ScalarsMapping } from './scalars/index.js';
import { invokeMethod } from '../../../shared.js';
import { generateSchemaName, generateZodInferTypeAlias } from './shared.js';

function generateZodInputTypeSpec(
    scalarsMapping: ScalarsMapping,
    type: z.infer<typeof inputTypeSchema>
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
        return getScalarSpecFromMapping(scalarsMapping, type.name).inputSchema
    }
    case 'InputType': {
        return ts.factory.createIdentifier(generateSchemaName(type.name))
    }
    }
}

function generateZodInputFieldSpec(
    scalarsMapping: ScalarsMapping,
    field: z.infer<typeof inputFieldSchema>
) {
    let expression: ts.Expression
    switch (field.spec._type) {
    case 'array': {
        expression = ts.factory.createCallExpression(
            ts.factory.createPropertyAccessExpression(
                ts.factory.createIdentifier('z'),
                'array'
            ),
            undefined,
            [generateZodInputTypeSpec(scalarsMapping, field.spec.type)]
        )
        break;
    }
    case 'literal': {
        expression = generateZodInputTypeSpec(scalarsMapping, field.spec.type)
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

function isFieldLazy(spec: z.infer<typeof inputFieldSpecSchema>): boolean {
    return spec.type._type === 'InputType'
}

export function generateInputTypeDefinitionFields(
    scalarsMapping: ScalarsMapping,
    fields: Record<string, z.infer<typeof inputFieldSchema>>
): (ts.PropertyAssignment | ts.GetAccessorDeclaration)[] {
    return Object.entries(fields).map(([name, field]) => {
        const fieldSpec = generateZodInputFieldSpec(scalarsMapping, field)
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
        return ts.factory.createPropertyAssignment(name, fieldSpec)
    })
}

function generateZodInputTypeDefinition(
    scalarsMapping: ScalarsMapping,
    name: string,
    fields: Record<string, z.infer<typeof inputFieldSchema>>
): ts.Node {
    return ts.factory.createVariableStatement(
        [ts.factory.createToken(ts.SyntaxKind.ExportKeyword)],
        ts.factory.createVariableDeclarationList([
            ts.factory.createVariableDeclaration(
                generateSchemaName(name),
                undefined,
                undefined,
                ts.factory.createCallExpression(
                    ts.factory.createPropertyAccessExpression(
                        ts.factory.createIdentifier('z'),
                        ts.factory.createIdentifier('object')
                    ),
                    undefined,
                    [ts.factory.createObjectLiteralExpression(
                        generateInputTypeDefinitionFields(
                            scalarsMapping,
                            fields
                        ),
                        true
                    )]
                )
            )
        ],
        ts.NodeFlags.Const),
    )
}

export function generateInputTypeDefinitions(
    scalarsMapping: ScalarsMapping,
    input: z.infer<typeof inputSchema>
): ts.Node[] {
    return [
        generateZodInputTypeDefinition(
            scalarsMapping,
            input.name,
            input.fields
        ),
        generateZodInferTypeAlias(input.name, generateSchemaName(input.name)),
        ts.factory.createIdentifier('\n')
    ]
}
