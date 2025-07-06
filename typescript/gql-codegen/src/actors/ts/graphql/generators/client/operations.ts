/* eslint-disable max-lines */
import ts from 'typescript';
import { RootSchema } from '@/schema/root.js';
import { operationSchema } from '@/schema/client/operation.js';
import { z } from 'zod/v4';
import {
    extractFragmentSourceTextsInSpec,
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

function generateOperationDocumentNode(
    schema: RootSchema,
    operation: z.infer<typeof operationSchema>
) {
    return ts.factory.createVariableStatement(
        [ts.factory.createToken(ts.SyntaxKind.ExportKeyword)],
        ts.factory.createVariableDeclarationList(
            [ts.factory.createVariableDeclaration(
                ts.factory.createIdentifier(operation.name + 'Document'),
                undefined,
                undefined,
                ts.factory.createStringLiteral([
                    operation.sourceText,
                    ...extractFragmentSourceTextsInSpec(
                        schema,
                        operation.fragmentSpec as FragmentSpecSchemaType
                    )
                ].join('\n'))
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
        generateOperationDocumentNode(schema, operation),
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
