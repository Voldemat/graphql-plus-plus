import ts from 'typescript'
import { z } from 'zod/v4'
import { objectNonCallableFieldSpecSchema } from '@/schema/server.js'
import { inputFieldSpecSchema } from '@/schema/shared.js'
import { generateTypeReferenceNode } from '../shared.js'

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
