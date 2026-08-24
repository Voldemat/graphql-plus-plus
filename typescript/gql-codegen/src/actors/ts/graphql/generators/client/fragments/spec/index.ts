import { RootSchema } from '@/schema/root.js';
import { ScalarsMapping } from '../../../server/scalars/mapping.js';
import { FragmentSpecSchemaType } from '@/schema/client/fragment.js';
import { resolveSelections } from './shared.js';
import { generateZodObjectFragmentSpecCallExpression } from './object/index.js';
import { generateZodUnionFragmentSpecCallExpression } from './union/index.js';

export function generateZodFragmentSpecCallExpression(
    scalarsMapping: ScalarsMapping,
    schema: RootSchema,
    lazyFragmentsSet: Set<string>,
    spec: FragmentSpecSchemaType,
    typenameConfig?: Parameters<typeof resolveSelections>[1],
    insideLazy: boolean = false,
) {
    if (spec._type === 'ObjectFragmentSpec') {
        return generateZodObjectFragmentSpecCallExpression(
            scalarsMapping,
            schema,
            lazyFragmentsSet,
            schema.server.objects[spec.name],
            spec.selections,
            insideLazy,
            typenameConfig || { ensurePresent: true, optional: true },
        );
    }
    return generateZodUnionFragmentSpecCallExpression(
        scalarsMapping,
        schema,
        lazyFragmentsSet,
        spec,
        insideLazy,
    );
}
