import { z } from 'zod/v4';
import ts from 'typescript';
import { enumSchema } from '../../../../schema/server.js';

export function generateEnumDefinition(
    e: z.infer<typeof enumSchema>
) {
    return ts.factory.createEnumDeclaration(
        ts.factory.createModifiersFromModifierFlags(ts.ModifierFlags.Export),
        e.name,
        e.values.map(value => ts.factory.createEnumMember(value))
    )
}
