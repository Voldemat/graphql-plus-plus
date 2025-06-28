import { z } from 'zod';
import { unionSchema } from '../../../schema.js';
import ts from 'typescript';
import { generateTypeReferenceNode } from './shared.js';

export function generateUnionTypeDefinition(
    scalars: string[],
    union: z.infer<typeof unionSchema>
) {
    return ts.factory.createTypeAliasDeclaration(
        ts.factory.createModifiersFromModifierFlags(ts.ModifierFlags.Export),
        union.name,
        undefined,
        ts.factory.createUnionTypeNode(
            Object.keys(union.items).map(item => {
                return generateTypeReferenceNode(scalars, item)
            })
        )
    )
}
