import { z } from 'zod/v4';
import {
    objectConditionalSpreadSelection,
    typenameSelection,
    UnionSelection,
} from '@/schema/client/fragment.js';
import { RootSchema } from '@/schema/root.js';
import assert from 'assert';

export function resolveUnionSelections(
    schema: RootSchema,
    specSelections: UnionSelection[],
): [
    z.infer<typeof objectConditionalSpreadSelection>[],
    z.infer<typeof typenameSelection>[],
] {
    const typenameSelections: z.infer<typeof typenameSelection>[] = [];
    const objectSelections = specSelections
        .map((s) => {
            assert(s._type !== 'UnionConditionalSpreadSelection');
            if (s._type === 'TypenameField') {
                typenameSelections.push(s);
                return [];
            }
            if (s._type === 'ObjectConditionalSpreadSelection') return [s];
            const fragmentSpec = schema.client.fragments[s.fragment].spec;
            assert(fragmentSpec._type === 'UnionFragmentSpec');
            const [selections, tSelections] = resolveUnionSelections(
                schema,
                fragmentSpec.selections,
            );
            typenameSelections.push(...tSelections);
            return selections;
        })
        .flat();
    return [objectSelections, typenameSelections];
}
