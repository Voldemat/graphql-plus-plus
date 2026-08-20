/* oxlint-disable max-lines */
import ts from 'typescript';
import { Config } from '../../actor.js';
import { generateHookReturnType } from '../hook-return-type.js';

function generateSyncHookReturnType(config: Config): ts.TypeNode {
    return generateHookReturnType(config, 'SYNC', 'TVariables', 'TResult');
}

export function generateSyncHookType(config: Config): ts.TypeAliasDeclaration {
    return ts.factory.createTypeAliasDeclaration(
        [],
        config.sdk.syncHookTypeName,
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
            [
                ts.factory.createParameterDeclaration(
                    undefined,
                    undefined,
                    'variables',
                    undefined,
                    ts.factory.createTypeReferenceNode('TVariables'),
                ),
                ts.factory.createParameterDeclaration(
                    undefined,
                    undefined,
                    'requestContext',
                    undefined,
                    ts.factory.createTypeReferenceNode('TRequestContext'),
                ),
            ],
            generateSyncHookReturnType(config),
        ),
    );
}

export function generateSyncHookBuilder(
    config: Config,
): ts.FunctionDeclaration {
    return ts.factory.createFunctionDeclaration(
        undefined,
        undefined,
        config.sdk.syncHookBuilderName,
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
        ts.factory.createTypeReferenceNode(config.sdk.syncHookTypeName, [
            ts.factory.createTypeReferenceNode('TRequestContext'),
            ts.factory.createTypeReferenceNode('TVariables'),
            ts.factory.createTypeReferenceNode('TResult'),
        ]),
        ts.factory.createBlock([
            ts.factory.createReturnStatement(
                ts.factory.createArrowFunction(
                    undefined,
                    undefined,
                    [
                        ts.factory.createParameterDeclaration(
                            undefined,
                            undefined,
                            'variables',
                            undefined,
                            ts.factory.createTypeReferenceNode('TVariables'),
                        ),
                        ts.factory.createParameterDeclaration(
                            undefined,
                            undefined,
                            'requestContext',
                            undefined,
                            ts.factory.createTypeReferenceNode(
                                'TRequestContext',
                            ),
                        ),
                    ],
                    generateSyncHookReturnType(config),
                    ts.factory.createToken(
                        ts.SyntaxKind.EqualsGreaterThanToken,
                    ),
                    ts.factory.createCallExpression(
                        ts.factory.createIdentifier('useOperation'),
                        [],
                        [
                            ts.factory.createIdentifier('executor'),
                            ts.factory.createIdentifier('operation'),
                            ts.factory.createIdentifier('variables'),
                            ts.factory.createIdentifier('requestContext'),
                        ],
                    ),
                ),
            ),
        ]),
    );
}
