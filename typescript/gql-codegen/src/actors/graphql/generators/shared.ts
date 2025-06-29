import ts from 'typescript'

export function createTypenamePropertySignature(name: string) {
    return ts.factory.createPropertySignature(
        undefined,
        '__typename',
        ts.factory.createToken(ts.SyntaxKind.QuestionToken),
        ts.factory.createLiteralTypeNode(
            ts.factory.createStringLiteral(name)
        )
    )
}

export function generateScalarReference(name: string) {
    return ts.factory.createIndexedAccessTypeNode(
        ts.factory.createIndexedAccessTypeNode(
            ts.factory.createTypeReferenceNode('Scalars'),
            ts.factory.createLiteralTypeNode(
                ts.factory.createStringLiteral(name)
            )
        ),
        ts.factory.createLiteralTypeNode(
            ts.factory.createStringLiteral('output')
        )
    )
}

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

