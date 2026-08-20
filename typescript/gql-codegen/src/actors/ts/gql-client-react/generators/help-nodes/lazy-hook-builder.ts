import ts from 'typescript';
import { Config } from '../../actor.js';
import { generateHookReturnType } from '../hook-return-type.js';

function generateLazyHookReturnType(config: Config): ts.TypeNode {
    return generateHookReturnType(config, 'LAZY', 'TVariables', 'TResult');
}

export function generateLazyHookType(config: Config): ts.TypeAliasDeclaration {
    return ts.factory.createTypeAliasDeclaration(
        [],
        config.sdk.lazyHookTypeName,
        [
            ts.factory.createTypeParameterDeclaration(
                undefined,
                'TRequestContext',
                ts.factory.createTypeReferenceNode('RequestContext'),
            ),
            ts.factory.createTypeParameterDeclaration(undefined, 'TVariables'),
            ts.factory.createTypeParameterDeclaration(undefined, 'TResult'),
        ],
        ts.factory.createFunctionTypeNode(
            undefined,
            [],
            generateLazyHookReturnType(config),
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
                ts.factory.createTypeReferenceNode('IExecutor', [
                    ts.factory.createTypeReferenceNode('TRequestContext'),
                ]),
            ),
            ts.factory.createTypeParameterDeclaration(
                undefined,
                'TRequestContext',
                ts.factory.createTypeReferenceNode('RequestContext'),
            ),
            ts.factory.createTypeParameterDeclaration(undefined, 'TVariables'),
            ts.factory.createTypeParameterDeclaration(undefined, 'TResult'),
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
                ts.factory.createTypeReferenceNode('SyncOperation', [
                    ts.factory.createTypeReferenceNode('TVariables'),
                    ts.factory.createTypeReferenceNode('TResult'),
                ]),
            ),
        ],
        ts.factory.createTypeReferenceNode(config.sdk.lazyHookTypeName, [
            ts.factory.createTypeReferenceNode('TRequestContext'),
            ts.factory.createTypeReferenceNode('TVariables'),
            ts.factory.createTypeReferenceNode('TResult'),
        ]),
        ts.factory.createBlock([
            ts.factory.createReturnStatement(
                ts.factory.createArrowFunction(
                    undefined,
                    undefined,
                    [],
                    generateLazyHookReturnType(config),
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
