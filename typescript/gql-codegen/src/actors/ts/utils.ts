import ts from 'typescript';

export function generateImportDeclaration(
    modulePath: string,
    imports: (string | { name: string, alias: string })[]
): ts.ImportDeclaration {
    return ts.factory.createImportDeclaration(
        undefined,
        ts.factory.createImportClause(
            false,
            undefined,
            ts.factory.createNamedImports(imports.map(i =>
                ts.factory.createImportSpecifier(
                    true,
                    typeof i === 'string' ?
                        undefined :
                        ts.factory.createIdentifier(i.alias),
                    ts.factory.createIdentifier(
                        typeof i === 'string' ? i : i.name
                    )
                )))
        ),
        ts.factory.createStringLiteral(modulePath)
    )
}
