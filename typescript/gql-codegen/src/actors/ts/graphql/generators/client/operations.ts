/* eslint-disable max-lines */
import ts from 'typescript';
import { RootSchema } from '@/schema/root.js';
import { operationSchema } from '@/schema/client/operation.js';
import { z } from 'zod/v4';
import {
    extractFragmentSourceTextsInSpec,
    generateZodFragmentSpecCallExpression,
} from './fragments.js';
import { FragmentSpecSchemaType } from '@/schema/client/fragment.js';
import { inputFieldSchema } from '@/schema/shared.js';
import {
    generateSchemaName,
    generateZodInferTypeAlias
} from '../server/shared.js';
import { generateInputTypeDefinitionFields } from '../server/inputs.js';
import { ScalarsMapping } from '../server/scalars/mapping.js';

export function opTypeToName(
    type: z.infer<typeof operationSchema>['type']
): string {
    switch (type) {
    case 'QUERY': return 'Query'
    case 'MUTATION': return 'Mutation'
    case 'SUBSCRIPTION': return 'Subscription'
    }
}

function parametersToFields(
    parameters: Record<string, z.infer<typeof inputFieldSchema>>
) {
    return Object.fromEntries(Object.keys(parameters).map(name =>
        [name.slice(1), parameters[name]]))
}

function generateOperationNode(
    schema: RootSchema,
    operation: z.infer<typeof operationSchema>
) {
    return ts.factory.createVariableStatement(
        ts.factory.createModifiersFromModifierFlags(
            ts.ModifierFlags.Export
        ),
        ts.factory.createVariableDeclarationList(
            [ts.factory.createVariableDeclaration(
                ts.factory.createIdentifier(operation.name + 'Operation'),
                undefined,
                undefined,
                ts.factory.createSatisfiesExpression(
                    ts.factory.createAsExpression(
                        ts.factory.createObjectLiteralExpression([
                            ts.factory.createPropertyAssignment(
                                'name',
                                ts.factory.createStringLiteral(operation.name),
                            ),
                            ts.factory.createPropertyAssignment(
                                'type',
                                ts.factory.createStringLiteral(operation.type)
                            ),
                            ts.factory.createPropertyAssignment(
                                'document',
                                ts.factory.createStringLiteral([
                                    operation.sourceText,
                                    ...extractFragmentSourceTextsInSpec(
                                        schema,
                                operation.fragmentSpec as FragmentSpecSchemaType
                                    )
                                ].join(' '))
                            ),
                            ts.factory.createPropertyAssignment(
                                'variablesSchema',
                                ts.factory.createIdentifier(
                                    generateSchemaName(
                                        operation.name + 'Variables'
                                    )
                                )
                            ),
                            ts.factory.createPropertyAssignment(
                                'resultSchema',
                                ts.factory.createIdentifier(
                                    generateSchemaName(
                                        operation.name + 'Result'
                                    )
                                )
                            )
                        ], true),
                        ts.factory.createTypeReferenceNode('const')
                    ),
                    ts.factory.createTypeReferenceNode('Operation', [
                        ts.factory.createTypeReferenceNode(
                            operation.name + 'Variables'
                        ),
                        ts.factory.createTypeReferenceNode(
                            operation.name + 'Result'
                        ),
                    ])
                )
            )],
            ts.NodeFlags.Const
        )
    )
}

function generateOperationZodInputSchema(
    scalarsMapping: ScalarsMapping,
    operation: z.infer<typeof operationSchema>
): ts.VariableStatement {
    return ts.factory.createVariableStatement(
        [ts.factory.createToken(ts.SyntaxKind.ExportKeyword)],
        ts.factory.createVariableDeclarationList(
            [ts.factory.createVariableDeclaration(
                ts.factory.createIdentifier(
                    generateSchemaName(operation.name + 'Variables')
                ),
                undefined,
                undefined,
                ts.factory.createCallExpression(
                    ts.factory.createPropertyAccessExpression(
                        ts.factory.createIdentifier('z'),
                        'object'
                    ),
                    undefined,
                    [ts.factory.createObjectLiteralExpression(
                        generateInputTypeDefinitionFields(
                            scalarsMapping,
                            parametersToFields(operation.parameters)
                        ), true
                    )]
                )
            )],
            ts.NodeFlags.Const
        )
    )
}

function genearteOperationZodOutputSchema(
    scalarsMapping: ScalarsMapping,
    schema: RootSchema,
    operation: z.infer<typeof operationSchema>
): ts.VariableStatement {
    return ts.factory.createVariableStatement(
        [ts.factory.createToken(ts.SyntaxKind.ExportKeyword)],
        ts.factory.createVariableDeclarationList(
            [ts.factory.createVariableDeclaration(
                ts.factory.createIdentifier(
                    generateSchemaName(operation.name + 'Result')
                ),
                undefined,
                undefined,
                generateZodFragmentSpecCallExpression(
                    scalarsMapping,
                    schema,
                    operation.fragmentSpec
                )
            )],
            ts.NodeFlags.Const
        )
    )
}

function generateOperationNodes(
    scalarsMapping: ScalarsMapping,
    schema: RootSchema,
    operation: z.infer<typeof operationSchema>
): ts.Node[] {
    return [
        generateOperationZodInputSchema(scalarsMapping, operation),
        generateZodInferTypeAlias(
            operation.name + 'Variables',
            generateSchemaName(operation.name + 'Variables')
        ),
        ts.factory.createIdentifier('\n'),
        genearteOperationZodOutputSchema(scalarsMapping, schema, operation),
        generateZodInferTypeAlias(
            operation.name + 'Result',
            generateSchemaName(operation.name + 'Result')
        ),
        generateOperationNode(schema, operation),
        ts.factory.createIdentifier('\n')
    ]
}

export function generateOperationsNodes(
    scalarsMapping: ScalarsMapping,
    schema: RootSchema,
): ts.Node[] {
    return Object.values(schema.client.operations).map(operation => {
        return generateOperationNodes(scalarsMapping, schema, operation)
    }).flat()
}
