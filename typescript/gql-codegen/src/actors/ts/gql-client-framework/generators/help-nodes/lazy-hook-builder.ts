/* oxlint-disable max-lines */
import ts from 'typescript';
import { Config } from '../../actor.js';

function generateLazyHookReturnType(
    variablesTypeNode: ts.TypeNode,
    resultTypeNode: ts.TypeNode,
): ts.TypeNode {
    return ts.factory.createTypeReferenceNode(
        'UseLazyOperationHookReturnType',
        [
            ts.factory.createTypeReferenceNode('TRequestContext'),
            variablesTypeNode,
            resultTypeNode,
        ],
    );
}

export function generateLazyHookType(config: Config): ts.TypeAliasDeclaration {
    return ts.factory.createTypeAliasDeclaration(
        [],
        config.sdk.lazyHookTypeName,
        [
            ts.factory.createTypeParameterDeclaration(
                undefined,
                'TRequestContext',
                ts.factory.createTypeReferenceNode('types.RequestContext'),
            ),
            ts.factory.createTypeParameterDeclaration(undefined, 'TVariables'),
            ts.factory.createTypeParameterDeclaration(undefined, 'TResult'),
        ],
        ts.factory.createFunctionTypeNode(
            undefined,
            [],
            generateLazyHookReturnType(
                ts.factory.createTypeReferenceNode('TVariables'),
                ts.factory.createTypeReferenceNode('TResult'),
            ),
        ),
    );
}

export function generateLazyHookBuilder(
    config: Config,
): ts.FunctionDeclaration {
    return ts.factory.createFunctionDeclaration(
        undefined,
        undefined,
        config.sdk.lazyHookBuilderName,
        [
            ts.factory.createTypeParameterDeclaration(
                undefined,
                'TExecutor',
                ts.factory.createTypeReferenceNode('types.IExecutor', [
                    ts.factory.createTypeReferenceNode('TRequestContext'),
                ]),
            ),
            ts.factory.createTypeParameterDeclaration(
                undefined,
                'TRequestContext',
                ts.factory.createTypeReferenceNode('types.RequestContext'),
            ),
            ts.factory.createTypeParameterDeclaration(
                undefined,
                'TOperation',
                ts.factory.createTypeReferenceNode('types.SyncOperation', [
                    ts.factory.createKeywordTypeNode(
                        ts.SyntaxKind.UnknownKeyword,
                    ),
                    ts.factory.createKeywordTypeNode(
                        ts.SyntaxKind.UnknownKeyword,
                    ),
                ]),
            ),
        ],
        [
            ts.factory.createParameterDeclaration(
                undefined,
                undefined,
                'executor',
                undefined,
                ts.factory.createTypeReferenceNode('TExecutor'),
            ),
            ts.factory.createParameterDeclaration(
                undefined,
                undefined,
                'operation',
                undefined,
                ts.factory.createTypeReferenceNode('TOperation'),
            ),
        ],
        ts.factory.createTypeReferenceNode(config.sdk.lazyHookTypeName, [
            ts.factory.createTypeReferenceNode('TRequestContext'),
            ts.factory.createTypeReferenceNode('types.OperationVariables', [
                ts.factory.createTypeReferenceNode('TOperation'),
            ]),
            ts.factory.createTypeReferenceNode('types.OperationResult', [
                ts.factory.createTypeReferenceNode('TOperation'),
            ]),
        ]),
        ts.factory.createBlock([
            ts.factory.createReturnStatement(
                ts.factory.createArrowFunction(
                    undefined,
                    undefined,
                    [],
                    generateLazyHookReturnType(
                        ts.factory.createTypeReferenceNode(
                            'types.OperationVariables',
                            [ts.factory.createTypeReferenceNode('TOperation')],
                        ),
                        ts.factory.createTypeReferenceNode(
                            'types.OperationResult',
                            [ts.factory.createTypeReferenceNode('TOperation')],
                        ),
                    ),
                    ts.factory.createToken(
                        ts.SyntaxKind.EqualsGreaterThanToken,
                    ),
                    ts.factory.createCallExpression(
                        ts.factory.createIdentifier('useLazyOperation'),
                        [],
                        [
                            ts.factory.createIdentifier('executor'),
                            ts.factory.createIdentifier('operation'),
                        ],
                    ),
                ),
            ),
        ]),
    );
}
