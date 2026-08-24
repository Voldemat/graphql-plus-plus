import { unionFragmentSpec } from '@/schema/client/fragment.js';
import { RootSchema } from '@/schema/root.js';
import ts from 'typescript';
import { z } from 'zod/v4';
import { ScalarsMapping } from '../../../../server/scalars/mapping.js';
import { generateZodObjectFragmentSpecCallExpression } from '../object/index.js';
import { resolveUnionSelections } from './selections.js';

export function generateZodUnionFragmentSpecCallExpression(
    scalarsMapping: ScalarsMapping,
    schema: RootSchema,
    lazyFragmentsSet: Set<string>,
    spec: z.infer<typeof unionFragmentSpec>,
    insideLazy: boolean,
): [ts.CallExpression, boolean] {
    const [objectSelections, typenameSelections] = resolveUnionSelections(
        schema,
        spec.selections,
    );
    for (const item of Object.keys(schema.server.unions[spec.name].items)) {
        if (!objectSelections.some((s) => s.object === item)) {
            objectSelections.push({
                _type: 'ObjectConditionalSpreadSelection',
                object: item,
                spec: {
                    _type: 'ObjectFragmentSpec',
                    name: item,
                    selections: [],
                },
            });
        }
    }
    let expression = ts.factory.createCallExpression(
        ts.factory.createPropertyAccessExpression(
            ts.factory.createIdentifier('z'),
            'union',
        ),
        undefined,
        [
            ts.factory.createArrayLiteralExpression(
                objectSelections.map(
                    (s) =>
                        generateZodObjectFragmentSpecCallExpression(
                            scalarsMapping,
                            schema,
                            lazyFragmentsSet,
                            schema.server.objects[s.object],
                            [...s.spec.selections, ...typenameSelections],
                            true,
                            { ensurePresent: true, optional: false },
                        )[0],
                ),
                true,
            ),
        ],
    );
    if (!insideLazy) {
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
    return [expression, !insideLazy];
}
