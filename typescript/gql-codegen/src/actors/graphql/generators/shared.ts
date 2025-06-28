import ts from 'typescript'
import { generateScalarReference } from './scalars/generators.js'
import {
    inputFieldSpecSchema,
    objectNonCallableFieldSpecSchema
} from '../../../schema.js'
import { z } from 'zod'

export function generateTypeReferenceNode(
    scalars: string[],
    name: string,
) {
    if (scalars.includes(name))
        return generateScalarReference(name)
    return ts.factory.createTypeReferenceNode(name)
}

export function createQuestionTokenIfNullable(nullable: boolean) {
    return nullable ?
        ts.factory.createToken(ts.SyntaxKind.QuestionToken) :
        undefined
}

export function generateNonCallableFieldSpec(
    scalars: string[],
    spec: z.infer<typeof objectNonCallableFieldSpecSchema> |
    z.infer<typeof inputFieldSpecSchema>,
) {
    switch (spec._type) {
    case 'array':
        return ts.factory.createArrayTypeNode(
            generateTypeReferenceNode(scalars, spec.type.name)
        )
    case 'literal':
        return generateTypeReferenceNode(scalars, spec.type.name)
    }
}


export function wrapInMaybeIfNullable(spec: ts.TypeNode, nullable: boolean) {
    return nullable ?
        ts.factory.createTypeReferenceNode(
            'Maybe',
            [spec]
        ) :
        spec
}
