import { z } from 'zod';
import ts from 'typescript';
import {
    objectFieldSpecSchema,
    objectSchema
} from '../../../schema.js';
import {
    createQuestionTokenIfNullable,
    generateNonCallableFieldSpec,
    wrapInMaybeIfNullable,
} from './shared.js';

function generateObjectFieldSpec(
    scalars: string[],
    spec: z.infer<typeof objectFieldSpecSchema>
) {
    if (spec._type === 'callable') {
        return generateNonCallableFieldSpec(scalars, spec.returnType)
    }
    return generateNonCallableFieldSpec(scalars, spec)
}

function createTypenamePropertySignature(name: string) {
    return ts.factory.createPropertySignature(
        undefined,
        '__typename',
        ts.factory.createToken(ts.SyntaxKind.QuestionToken),
        ts.factory.createLiteralTypeNode(
            ts.factory.createStringLiteral(name)
        )
    )
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
