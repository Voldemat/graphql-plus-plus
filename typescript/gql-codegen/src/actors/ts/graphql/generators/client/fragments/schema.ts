import { fragmentSchema } from '@/schema/client/fragment.js';
import { RootSchema } from '@/schema/root.js';
import ts from 'typescript';
import { z } from 'zod/v4';
import { ScalarsMapping } from '../../server/scalars/mapping.js';
import { generateSchemaName } from '../../server/shared.js';
import { generateZodFragmentSpecCallExpression } from './spec/index.js';

export function generateZodFragmentSchema(
    scalarsMapping: ScalarsMapping,
    schema: RootSchema,
    lazyFragmentsSet: Set<string>,
    fragmentName: string,
    fragment: z.infer<typeof fragmentSchema>,
): [ts.VariableStatement, boolean] {
    const [expression, isLazy] = generateZodFragmentSpecCallExpression(
        scalarsMapping,
        schema,
        lazyFragmentsSet,
        fragment.spec,
    );
    return [
        ts.factory.createVariableStatement(
            [ts.factory.createToken(ts.SyntaxKind.ExportKeyword)],
            ts.factory.createVariableDeclarationList(
                [
                    ts.factory.createVariableDeclaration(
                        ts.factory.createIdentifier(
                            generateSchemaName(fragmentName + 'Fragment'),
                        ),
                        undefined,
                        undefined,
                        expression,
                    ),
                ],
                ts.NodeFlags.Const,
            ),
        ),
        isLazy,
    ];
}
