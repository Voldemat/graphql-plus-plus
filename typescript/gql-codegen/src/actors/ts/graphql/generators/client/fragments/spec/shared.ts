import { z } from 'zod/v4';
import {
    FragmentSpecSchemaType,
    objectSelection,
} from '@/schema/client/fragment.js';
import { RootSchema } from '@/schema/root.js';
import assert from 'assert';

export function resolveSelections(
    specSelections: z.infer<typeof objectSelection>[],
    typenameConfig:
        | { ensurePresent: boolean; optional: boolean }
        | { ignore: true },
) {
    const ignoreTypename = 'ignore' in typenameConfig;
    const selections = specSelections
        .filter((s) => s._type !== 'TypenameField' || !ignoreTypename)
        .toSorted((s1, s2) => s1._type.localeCompare(s2._type));
    if (ignoreTypename) return selections;
    const hasTypename = specSelections.some((s) => s._type === 'TypenameField');
    const hasSpreadSelection = specSelections.some(
        (s) => s._type === 'SpreadSelection',
    );
    if (!hasTypename) {
        if (hasSpreadSelection) {
            selections.push({ _type: 'TypenameField', alias: null });
        } else {
            selections.unshift({ _type: 'TypenameField', alias: null });
        }
    }
    return selections;
}

export function extractFragmentNamesInSpec(
    schema: RootSchema,
    fragmentSpec: FragmentSpecSchemaType,
): string[] {
    if (fragmentSpec._type === 'UnionFragmentSpec') {
        return fragmentSpec.selections
            .map((s): string[] => {
                if (s._type === 'SpreadSelection') {
                    const fragment = schema.client.fragments[s.fragment];
                    return [
                        s.fragment,
                        ...extractFragmentNamesInSpec(schema, fragment.spec),
                    ];
                }
                if (s._type === 'ObjectConditionalSpreadSelection') {
                    return extractFragmentNamesInSpec(schema, s.spec);
                }
                return [];
            })
            .flat();
    }
    return fragmentSpec.selections
        .map((s): string[] => {
            if (s._type === 'SpreadSelection') {
                const fragment = schema.client.fragments[s.fragment];
                assert(fragment.spec !== undefined);
                return [
                    s.fragment,
                    ...extractFragmentNamesInSpec(schema, fragment.spec),
                ];
            }
            if (s._type === 'FieldSelection' && s.selection != null) {
                assert(s.selection !== undefined);
                return extractFragmentNamesInSpec(
                    schema,
                    s.selection as FragmentSpecSchemaType,
                );
            }
            return [];
        })
        .flat();
}

export function extractFragmentSourceTextsInSpec(
    schema: RootSchema,
    fragmentSpec: FragmentSpecSchemaType,
): string[] {
    return Array.from(
        new Set(extractFragmentNamesInSpec(schema, fragmentSpec)),
    ).map((f) => schema.client.fragments[f].sourceText);
}
