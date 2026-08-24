import { objectSelection } from '@/schema/client/fragment.js';
import { RootSchema } from '@/schema/root.js';
import { objectSchema } from '@/schema/server.js';
import ts from 'typescript';
import { z } from 'zod/v4';
import { ScalarsMapping } from '../../../../server/scalars/mapping.js';
import { resolveSelections } from '../shared.js';
import { generateFieldSelection } from './field_selection.js';
import { generateSpreadSelection } from './spread_selection.js';
import { generateTypenameFieldSelection } from './typename_field.js';

export function generateZodObjectSelection(
    scalarsMapping: ScalarsMapping,
    schema: RootSchema,
    lazyFragmentsSet: Set<string>,
    objectType: z.infer<typeof objectSchema>,
    selection: z.infer<typeof objectSelection>,
    insideLazy: boolean,
    typenameConfig: Parameters<typeof resolveSelections>[1],
): [ts.PropertyAssignment | ts.SpreadAssignment | null, boolean] {
    switch (selection._type) {
        case 'TypenameField': {
            return [
                generateTypenameFieldSelection(
                    typenameConfig,
                    objectType.name,
                    selection,
                ),
                false,
            ];
        }
        case 'FieldSelection': {
            return [
                generateFieldSelection(
                    scalarsMapping,
                    schema,
                    lazyFragmentsSet,
                    insideLazy,
                    objectType,
                    selection,
                ),
                false,
            ];
        }
        case 'SpreadSelection': {
            return [
                generateSpreadSelection(
                    selection.fragment,
                    lazyFragmentsSet.has(selection.fragment),
                ),
                true,
            ];
        }
    }
}
