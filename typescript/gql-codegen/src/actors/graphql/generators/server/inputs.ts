import { z } from 'zod/v4';
import ts from 'typescript';
import {
    generateNonCallableFieldSpec
} from './shared.js';
import { inputSchema } from '../../../../schema/server.js';
import { createQuestionTokenIfNullable } from '../shared.js';

export function generateInputTypeDefinition(
    scalars: string[],
    input: z.infer<typeof inputSchema>
) {
    return ts.factory.createInterfaceDeclaration(
        ts.factory.createModifiersFromModifierFlags(
            ts.ModifierFlags.Export
        ),
        input.name,
        undefined,
        undefined,
        Object.entries(input.fields).map(([name, field]) => {
            const spec = generateNonCallableFieldSpec(scalars, field.spec)
            return ts.factory.createPropertySignature(
                undefined,
                name,
                createQuestionTokenIfNullable(field.nullable),
                field.nullable ?
                    ts.factory.createTypeReferenceNode(
                        'Maybe',
                        [spec]
                    ) :
                    spec
            )
        })
    )
}
