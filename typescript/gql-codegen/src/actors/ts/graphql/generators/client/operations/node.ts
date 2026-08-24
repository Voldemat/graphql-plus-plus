import { ClientTypeNameBuilders } from '@/actors/ts/shared.js';
import { FragmentSpecSchemaType } from '@/schema/client/fragment.js';
import { Operation, operationSchema } from '@/schema/client/operation.js';
import { RootSchema } from '@/schema/root.js';
import ts from 'typescript';
import { z } from 'zod/v4';
import { generateSchemaName } from '../../server/shared.js';
import { extractFragmentSourceTextsInSpec } from '../fragments/spec/shared.js';

function generateProperties(
    clientTypeNameBuilders: ClientTypeNameBuilders,
    schema: RootSchema,
    operation: Operation,
): ts.PropertyAssignment[] {
    return [
        ts.factory.createPropertyAssignment(
            'name',
            ts.factory.createStringLiteral(operation.name),
        ),
        ts.factory.createPropertyAssignment(
            'type',
            ts.factory.createStringLiteral(operation.type),
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
                    clientTypeNameBuilders.variablesTypeName(operation.name),
                ),
            ),
        ),
        ts.factory.createPropertyAssignment(
            'resultSchema',
            ts.factory.createIdentifier(
                generateSchemaName(
                    clientTypeNameBuilders.resultTypeName(operation.name),
                ),
            ),
        ),
    ];
}

export function generateOperationNode(
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
                    ts.factory.createAsExpression(
                        ts.factory.createObjectLiteralExpression(
                            generateProperties(
                                clientTypeNameBuilders,
                                schema,
                                operation,
                            ),
                            true,
                        ),
                        ts.factory.createTypeReferenceNode('const'),
                    ),
                ),
            ],
            ts.NodeFlags.Const,
        ),
    );
}
