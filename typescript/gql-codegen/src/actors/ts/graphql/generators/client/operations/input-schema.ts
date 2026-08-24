import { operationSchema } from '@/schema/client/operation.js';
import { inputFieldSchema } from '@/schema/shared.js';
import ts from 'typescript';
import { z } from 'zod/v4';
import { generateInputTypeDefinitionFields } from '../../server/inputs.js';
import { ScalarsMapping } from '../../server/scalars/mapping.js';
import { generateSchemaName } from '../../server/shared.js';

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

export function generateOperationZodInputSchema(
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
                                    true,
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
