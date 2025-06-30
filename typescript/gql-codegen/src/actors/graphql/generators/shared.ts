import ts from 'typescript'

export function createQuestionTokenIfNullable(nullable: boolean) {
    return nullable ?
        ts.factory.createToken(ts.SyntaxKind.QuestionToken) :
        undefined
}

export function createTypenamePropertySignature(
    name: string,
    optional: boolean,
    alias: string | null,
) {
    return ts.factory.createPropertySignature(
        undefined,
        alias || '__typename',
        createQuestionTokenIfNullable(optional),
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

export function generateStringOrTemplate(value: string, values: string[]) {
    if (values.length === 0) return ts.factory.createStringLiteral(value)
    return ts.factory.createTemplateExpression(
        ts.factory.createTemplateHead(value),
        values.map((item, index) =>
            ts.factory.createTemplateSpan(
                ts.factory.createIdentifier(item),
                index + 1 !== values.length ?
                    ts.factory.createTemplateMiddle('', '') :
                    ts.factory.createTemplateTail('', '')
            ))
    )
}
