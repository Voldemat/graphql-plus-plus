import { objectSelection } from '@/schema/client/fragment.js';
import { RootSchema } from '@/schema/root.js';
import { objectSchema } from '@/schema/server.js';
import ts from 'typescript';
import { z } from 'zod/v4';
import { ScalarsMapping } from '../../../../server/scalars/mapping.js';
import { resolveSelections } from '../shared.js';
import { generateZodObjectSelection } from './selection.js';

export function generateZodObjectFragmentSpecCallExpression(
    scalarsMapping: ScalarsMapping,
    schema: RootSchema,
    lazyFragmentsSet: Set<string>,
    object: z.infer<typeof objectSchema>,
    specSelections: z.infer<typeof objectSelection>[],
    insideLazy: boolean,
    typenameConfig: Parameters<typeof resolveSelections>[1],
): [ts.CallExpression, boolean] {
    const selections = resolveSelections(specSelections, typenameConfig);
    let needsLazy = false;
    const properties = selections
        .map((s) => {
            const [property, pNeedsLazy] = generateZodObjectSelection(
                scalarsMapping,
                schema,
                lazyFragmentsSet,
                object,
                s,
                insideLazy,
                typenameConfig,
            );
            if (needsLazy === false && pNeedsLazy) {
                needsLazy = true;
            }
            return property;
        })
        .filter((s) => s !== null);
    let expression = ts.factory.createCallExpression(
        ts.factory.createPropertyAccessExpression(
            ts.factory.createIdentifier('z'),
            'object',
        ),
        undefined,
        [ts.factory.createObjectLiteralExpression(properties, true)],
    );
    if (needsLazy) {
        expression = ts.factory.createCallExpression(
            ts.factory.createPropertyAccessExpression(
                ts.factory.createIdentifier('z'),
                'lazy',
            ),
            undefined,
            [
                ts.factory.createArrowFunction(
                    undefined,
                    undefined,
                    [],
                    undefined,
                    ts.factory.createToken(
                        ts.SyntaxKind.EqualsGreaterThanToken,
                    ),
                    expression,
                ),
            ],
        );
    }
    return [expression, needsLazy];
}
