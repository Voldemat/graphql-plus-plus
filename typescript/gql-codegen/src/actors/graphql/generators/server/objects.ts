import { z } from 'zod/v4';
import ts from 'typescript';
import {
    generateNonCallableFieldSpec,
    wrapInMaybeIfNullable,
} from './shared.js';
import {
    objectFieldSpecSchema,
    objectSchema
} from '../../../../schema/server.js';
import {
    createQuestionTokenIfNullable,
    createTypenamePropertySignature
} from '../shared.js';

function generateObjectFieldSpec(
    scalars: string[],
    spec: z.infer<typeof objectFieldSpecSchema>
) {
    if (spec._type === 'callable') {
        return generateNonCallableFieldSpec(scalars, spec.returnType)
    }
    return generateNonCallableFieldSpec(scalars, spec)
}

export function generateObjectInterfaceDefinition(
    scalars: string[],
    object: z.infer<typeof objectSchema>
) {
    return ts.factory.createInterfaceDeclaration(
        ts.factory.createModifiersFromModifierFlags(
            ts.ModifierFlags.Export
        ),
        object.name,
        undefined,
        undefined,
        [
            createTypenamePropertySignature(object.name),
            ...Object.entries(object.fields).map(([name, field]) => {
                const spec = generateObjectFieldSpec(scalars, field.spec)
                return ts.factory.createPropertySignature(
                    undefined,
                    name,
                    createQuestionTokenIfNullable(field.nullable),
                    wrapInMaybeIfNullable(spec, field.nullable)
                )
            })
        ]
    )
}
