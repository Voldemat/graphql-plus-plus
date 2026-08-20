import ts from 'typescript';

export function generateImportSpecifier(
    name: string,
    typeOnly: boolean = false,
    alias: string | null = null,
): ts.ImportSpecifier {
    return ts.factory.createImportSpecifier(
        typeOnly,
        alias === null ? undefined : ts.factory.createIdentifier(alias),
        ts.factory.createIdentifier(name),
    );
}

export function generateImportDeclaration(
    modulePath: string,
    imports: ts.ImportSpecifier[],
): ts.ImportDeclaration {
    return ts.factory.createImportDeclaration(
        undefined,
        ts.factory.createImportClause(
            false,
            undefined,
            ts.factory.createNamedImports(imports),
        ),
        ts.factory.createStringLiteral(modulePath),
    );
}
