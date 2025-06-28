import { z } from 'zod';
import { inputSchema } from '../../../schema.js';
import ts from 'typescript';
import {
    createQuestionTokenIfNullable,
    generateNonCallableFieldSpec
} from './shared.js';

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
