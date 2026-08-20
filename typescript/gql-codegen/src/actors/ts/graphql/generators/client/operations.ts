/* eslint-disable max-lines */
import { ClientTypeNameBuilders } from '@/actors/ts/shared.js';
import { FragmentSpecSchemaType } from '@/schema/client/fragment.js';
import { operationSchema } from '@/schema/client/operation.js';
import { RootSchema } from '@/schema/root.js';
import { inputFieldSchema } from '@/schema/shared.js';
import ts from 'typescript';
import { z } from 'zod/v4';
import { generateInputTypeDefinitionFields } from '../server/inputs.js';
import { ScalarsMapping } from '../server/scalars/mapping.js';
import {
    generateSchemaName,
    generateZodInferInterfaceType,
} from '../server/shared.js';
import {
    extractFragmentSourceTextsInSpec,
    generateZodFragmentSpecCallExpression,
} from './fragments.js';

export function opTypeToName(
    type: z.infer<typeof operationSchema>['type'],
): string {
    switch (type) {
        case 'QUERY':
            return 'Query';
        case 'MUTATION':
            return 'Mutation';
        case 'SUBSCRIPTION':
            return 'Subscription';
    }
}

function parametersToFields(
    parameters: Record<string, z.infer<typeof inputFieldSchema>>,
) {
    return Object.fromEntries(
        Object.keys(parameters).map((name) => [
            name.slice(1),
            parameters[name],
        ]),
    );
}

function generateOperationNode(
    clientTypeNameBuilders: ClientTypeNameBuilders,
    schema: RootSchema,
    operation: z.infer<typeof operationSchema>,
) {
    return ts.factory.createVariableStatement(
        ts.factory.createModifiersFromModifierFlags(ts.ModifierFlags.Export),
        ts.factory.createVariableDeclarationList(
            [
                ts.factory.createVariableDeclaration(
                    ts.factory.createIdentifier(
                        clientTypeNameBuilders.operationTypeName(
                            operation.name,
                        ),
                    ),
                    undefined,
                    undefined,
                    ts.factory.createSatisfiesExpression(
                        ts.factory.createAsExpression(
                            ts.factory.createObjectLiteralExpression(
                                [
                                    ts.factory.createPropertyAssignment(
                                        'name',
                                        ts.factory.createStringLiteral(
                                            operation.name,
                                        ),
                                    ),
                                    ts.factory.createPropertyAssignment(
                                        'type',
                                        ts.factory.createStringLiteral(
                                            operation.type,
                                        ),
                                    ),
                                    ts.factory.createPropertyAssignment(
                                        'document',
                                        ts.factory.createStringLiteral(
                                            [
                                                operation.sourceText,
                                                ...extractFragmentSourceTextsInSpec(
                                                    schema,
                                                    operation.fragmentSpec as FragmentSpecSchemaType,
                                                ),
                                            ].join(' '),
                                        ),
                                    ),
                                    ts.factory.createPropertyAssignment(
                                        'variablesSchema',
                                        ts.factory.createIdentifier(
                                            generateSchemaName(
                                                clientTypeNameBuilders.variablesTypeName(
                                                    operation.name,
                                                ),
                                            ),
                                        ),
                                    ),
                                    ts.factory.createPropertyAssignment(
                                        'resultSchema',
                                        ts.factory.createIdentifier(
                                            generateSchemaName(
                                                clientTypeNameBuilders.resultTypeName(
                                                    operation.name,
                                                ),
                                            ),
                                        ),
                                    ),
                                ],
                                true,
                            ),
                            ts.factory.createTypeReferenceNode('const'),
                        ),
                        ts.factory.createTypeReferenceNode('Operation', [
                            ts.factory.createTypeReferenceNode(
                                clientTypeNameBuilders.variablesTypeName(
                                    operation.name,
                                ),
                            ),
                            ts.factory.createTypeReferenceNode(
                                clientTypeNameBuilders.resultTypeName(
                                    operation.name,
                                ),
                            ),
                        ]),
                    ),
                ),
            ],
            ts.NodeFlags.Const,
        ),
    );
}

function generateOperationZodInputSchema(
    scalarsMapping: ScalarsMapping,
    operation: z.infer<typeof operationSchema>,
    variablesName: string,
): ts.VariableStatement {
    return ts.factory.createVariableStatement(
        [ts.factory.createToken(ts.SyntaxKind.ExportKeyword)],
        ts.factory.createVariableDeclarationList(
            [
                ts.factory.createVariableDeclaration(
                    ts.factory.createIdentifier(
                        generateSchemaName(variablesName),
                    ),
                    undefined,
                    undefined,
                    ts.factory.createCallExpression(
                        ts.factory.createPropertyAccessExpression(
                            ts.factory.createIdentifier('z'),
                            'object',
                        ),
                        undefined,
                        [
                            ts.factory.createObjectLiteralExpression(
                                generateInputTypeDefinitionFields(
                                    scalarsMapping,
                                    parametersToFields(operation.parameters),
                                ),
                                true,
                            ),
                        ],
                    ),
                ),
            ],
            ts.NodeFlags.Const,
        ),
    );
}

function genearteOperationZodOutputSchema(
    scalarsMapping: ScalarsMapping,
    schema: RootSchema,
    operation: z.infer<typeof operationSchema>,
    resultName: string,
): ts.VariableStatement {
    return ts.factory.createVariableStatement(
        [ts.factory.createToken(ts.SyntaxKind.ExportKeyword)],
        ts.factory.createVariableDeclarationList(
            [
                ts.factory.createVariableDeclaration(
                    ts.factory.createIdentifier(generateSchemaName(resultName)),
                    undefined,
                    undefined,
                    generateZodFragmentSpecCallExpression(
                        scalarsMapping,
                        schema,
                        operation.fragmentSpec,
                    ),
                ),
            ],
            ts.NodeFlags.Const,
        ),
    );
}

function generateOperationNodes(
    clientTypeNameBuilders: ClientTypeNameBuilders,
    scalarsMapping: ScalarsMapping,
    schema: RootSchema,
    operation: z.infer<typeof operationSchema>,
): ts.Node[] {
    const variablesName = clientTypeNameBuilders.variablesTypeName(
        operation.name,
    );
    const resultName = clientTypeNameBuilders.resultTypeName(operation.name);
    return [
        generateOperationZodInputSchema(
            scalarsMapping,
            operation,
            variablesName,
        ),
        generateZodInferInterfaceType(
            'input',
            variablesName,
            generateSchemaName(variablesName),
        ),
        ts.factory.createIdentifier('\n'),
        genearteOperationZodOutputSchema(
            scalarsMapping,
            schema,
            operation,
            resultName,
        ),
        generateZodInferInterfaceType(
            'output',
            resultName,
            generateSchemaName(resultName),
        ),
        generateOperationNode(clientTypeNameBuilders, schema, operation),
        ts.factory.createIdentifier('\n'),
    ];
}

export function generateOperationsNodes(
    clientTypeNameBuilders: ClientTypeNameBuilders,
    scalarsMapping: ScalarsMapping,
    schema: RootSchema,
): ts.Node[] {
    return Object.values(schema.client.operations)
        .map((operation) => {
            return generateOperationNodes(
                clientTypeNameBuilders,
                scalarsMapping,
                schema,
                operation,
            );
        })
        .flat();
}
