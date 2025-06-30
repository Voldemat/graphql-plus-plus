import { z } from 'zod/v4';
import ts from 'typescript';
import {
    generateNonCallableFieldSpec
} from './shared.js';
import { inputSchema } from '../../../../schema/server.js';
import { createQuestionTokenIfNullable } from '../shared.js';
import { inputFieldSchema } from '../../../../schema/shared.js';

export function generateInputFieldsPropertySignatures(
    scalars: string[],
    fields: Record<string, z.infer<typeof inputFieldSchema>>
) {
    return Object.entries(fields).map(([name, field]) => {
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
}

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
        generateInputFieldsPropertySignatures(scalars, input.fields)
    )
}
