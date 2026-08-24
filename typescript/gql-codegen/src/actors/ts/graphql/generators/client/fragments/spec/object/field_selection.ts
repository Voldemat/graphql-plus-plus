import { invokeMethod } from '../../../../../../shared.js';
import { fieldSelection } from '@/schema/client/fragment.js';
import { RootSchema } from '@/schema/root.js';
import { objectSchema } from '@/schema/server.js';
import ts from 'typescript';
import { z } from 'zod/v4';
import { generateZodObjectFieldSpec } from '../../../../server/objects.js';
import { ScalarsMapping } from '../../../../server/scalars/mapping.js';
import { generateZodFragmentSpecCallExpression } from '../index.js';

export function generateFieldSelection(
    scalarsMapping: ScalarsMapping,
    schema: RootSchema,
    lazyFragmentsSet: Set<string>,
    insideLazy: boolean,
    objectType: z.infer<typeof objectSchema>,
    selection: z.infer<typeof fieldSelection>,
): ts.PropertyAssignment {
    const fieldSpec = objectType.fields[selection.name];
    let expression: ts.Expression;
    if (selection.selection == null) {
        expression = generateZodObjectFieldSpec(scalarsMapping, fieldSpec);
    } else {
        const optional =
            fieldSpec.spec._type !== 'callable' ||
            !selection.selection.selections.every(
                (s) => s._type === 'TypenameField',
            ) ||
            !fieldSpec.nullable;
        // eslint-disable-next-line no-use-before-define
        [expression] = generateZodFragmentSpecCallExpression(
            scalarsMapping,
            schema,
            lazyFragmentsSet,
            selection.selection,
            { ensurePresent: true, optional },
            insideLazy,
        );
        if (
            fieldSpec.spec._type === 'array' ||
            (fieldSpec.spec._type === 'callable' &&
                fieldSpec.spec.returnType._type === 'array')
        ) {
            expression = ts.factory.createCallExpression(
                ts.factory.createPropertyAccessExpression(
                    ts.factory.createIdentifier('z'),
                    'array',
                ),
                undefined,
                [expression],
            );
        }
    }
    if (fieldSpec.nullable) {
        expression = invokeMethod(
            invokeMethod(expression, 'nullable', []),
            'optional',
            [],
        );
    }
    return ts.factory.createPropertyAssignment(selection.alias, expression);
}
