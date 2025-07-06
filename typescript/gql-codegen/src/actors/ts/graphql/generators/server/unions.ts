import { z } from 'zod/v4';
import ts from 'typescript';
import { objectSchema, unionSchema } from '@/schema/server.js';
import { ScalarsMapping } from './scalars/mapping.js';
import { generateSchemaName, generateZodInferTypeAlias } from './shared.js';
import { generateZodObjectTypeExpression } from './objects.js';

function generateZodUnionTypeNode(
    scalarsMapping: ScalarsMapping,
    objects: Record<string, z.infer<typeof objectSchema>>,
    union: z.infer<typeof unionSchema>
) {
    return ts.factory.createVariableStatement(
        [ts.factory.createToken(ts.SyntaxKind.ExportKeyword)],
        ts.factory.createVariableDeclarationList([
            ts.factory.createVariableDeclaration(
                generateSchemaName(union.name),
                undefined,
                undefined,
                ts.factory.createCallExpression(
                    ts.factory.createPropertyAccessExpression(
                        ts.factory.createIdentifier('z'),
                        ts.factory.createIdentifier('discriminatedUnion')
                    ),
                    undefined,
                    [ts.factory.createStringLiteral(union.name),
                        ts.factory.createArrayLiteralExpression(
                            Object.keys(union.items)
                                .map(item =>
                                    generateZodObjectTypeExpression(
                                        scalarsMapping,
                                        objects[item],
                                        true
                                    )),
                            true
                        )
                    ]
                )
            )
        ],
        ts.NodeFlags.Const),
    )
}
export function generateUnionTypeDefinitions(
    scalarsMapping: ScalarsMapping,
    objects: Record<string, z.infer<typeof objectSchema>>,
    union: z.infer<typeof unionSchema>
): ts.Node[] {
    return [
        generateZodUnionTypeNode(scalarsMapping, objects, union),
        generateZodInferTypeAlias(union.name, generateSchemaName(union.name)),
        ts.factory.createIdentifier('\n')
    ]
}
