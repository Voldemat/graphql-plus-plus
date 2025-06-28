import { z } from 'zod';
import { enumSchema } from '../../../schema.js';
import ts from 'typescript';

export function generateEnumDefinition(
    e: z.infer<typeof enumSchema>
) {
    return ts.factory.createEnumDeclaration(
        ts.factory.createModifiersFromModifierFlags(ts.ModifierFlags.Export),
        e.name,
        e.values.map(value => ts.factory.createEnumMember(value))
    )
}
