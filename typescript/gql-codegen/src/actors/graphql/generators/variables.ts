import { z } from 'zod';
import { objectFieldSchema } from '../../../schema.js';
import ts from 'typescript';
import {
    createQuestionTokenIfNullable,
    generateNonCallableFieldSpec,
    wrapInMaybeIfNullable
} from './shared.js';


function generateVariablePropertySignatures(
    scalars: string[],
    field: z.infer<typeof objectFieldSchema>
) {
    if (field.spec._type !== 'callable') {
        return []
    }
    return Object.entries(field.spec.arguments).map(([name, argument]) => {
        const spec = generateNonCallableFieldSpec(scalars, argument.spec)
        return ts.factory.createPropertySignature(
            undefined,
            name,
            createQuestionTokenIfNullable(argument.nullable),
            wrapInMaybeIfNullable(spec, argument.nullable)
        )
    })
}
export function generateQueriesVariablesDefinitions(
    scalars: string[],
    queryName: string,
    field: z.infer<typeof objectFieldSchema>
): ts.TypeAliasDeclaration {
    return ts.factory.createTypeAliasDeclaration(
        ts.factory.createModifiersFromModifierFlags(
            ts.ModifierFlags.Export
        ),
        queryName + 'Variables',
        undefined,
        ts.factory.createTypeReferenceNode(
            'Exact',
            [ts.factory.createTypeLiteralNode(
                generateVariablePropertySignatures(scalars, field)
            )]
        )

    )
}
