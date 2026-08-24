import { operationSchema } from '@/schema/client/operation.js';
import { RootSchema } from '@/schema/root.js';
import ts from 'typescript';
import { z } from 'zod/v4';
import { ScalarsMapping } from '../../server/scalars/mapping.js';
import { generateSchemaName } from '../../server/shared.js';
import { generateZodFragmentSpecCallExpression } from '../fragments/spec/index.js';

export function generateOperationZodOutputSchema(
    scalarsMapping: ScalarsMapping,
    schema: RootSchema,
    lazyFragmentsSet: Set<string>,
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
                        lazyFragmentsSet,
                        operation.fragmentSpec,
                        undefined,
                        true,
                    )[0],
                ),
            ],
            ts.NodeFlags.Const,
        ),
    );
}
