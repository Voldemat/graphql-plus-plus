import { z } from 'zod/v4';
import { invokeMethod } from '../../../../../../shared.js';
import { resolveSelections } from '../shared.js';
import ts from 'typescript';
import { typenameSelection } from '@/schema/client/fragment.js';

export function generateTypenameFieldSelection(
    typenameConfig: Parameters<typeof resolveSelections>[1],
    objectTypeName: string,
    selection: z.infer<typeof typenameSelection>,
): ts.PropertyAssignment | null {
    if ('ignore' in typenameConfig) return null;
    let expression = invokeMethod(ts.factory.createIdentifier('z'), 'literal', [
        ts.factory.createStringLiteral(objectTypeName),
    ]);
    if (selection.alias === null && typenameConfig.optional) {
        expression = invokeMethod(
            invokeMethod(expression, 'nullable', []),
            'optional',
            [],
        );
    }
    return ts.factory.createPropertyAssignment(
        selection.alias || '__typename',
        expression,
    );
}
