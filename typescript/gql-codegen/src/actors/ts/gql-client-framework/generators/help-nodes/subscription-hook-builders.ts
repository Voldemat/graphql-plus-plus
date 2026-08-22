/* oxlint-disable max-lines */
import ts from 'typescript';
import { Config } from '../../actor.js';

function generateSubscriptionHookReturnType(
    resultTypeNode: ts.TypeNode,
): ts.TypeNode {
    return ts.factory.createTypeReferenceNode('UseSubscriptionHookReturnType', [
        resultTypeNode,
    ]);
}

export function generateSubscriptionHookType(
    config: Config,
): ts.TypeAliasDeclaration {
    return ts.factory.createTypeAliasDeclaration(
        [],
        config.sdk.subscriptionHookTypeName,
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
            [
                ts.factory.createParameterDeclaration(
                    undefined,
                    undefined,
                    'variables',
                    undefined,
                    config.sdk.buildVariablesType(
                        ts.factory.createTypeReferenceNode('TVariables'),
                    ),
                ),
                ts.factory.createParameterDeclaration(
                    undefined,
                    undefined,
                    'requestContext',
                    undefined,
                    ts.factory.createTypeReferenceNode('TRequestContext'),
                ),
            ],
            generateSubscriptionHookReturnType(
                ts.factory.createTypeReferenceNode('TResult'),
            ),
        ),
    );
}

export function generateSubscriptionHookBuilder(
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
                ts.factory.createTypeReferenceNode(
                    'types.SubscriptionOperation',
                    [
                        ts.factory.createKeywordTypeNode(
                            ts.SyntaxKind.UnknownKeyword,
                        ),
                        ts.factory.createKeywordTypeNode(
                            ts.SyntaxKind.UnknownKeyword,
                        ),
                    ],
                ),
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
        ts.factory.createTypeReferenceNode(
            config.sdk.subscriptionHookTypeName,
            [
                ts.factory.createTypeReferenceNode('TRequestContext'),
                ts.factory.createTypeReferenceNode('types.OperationVariables', [
                    ts.factory.createTypeReferenceNode('TOperation'),
                ]),
                ts.factory.createTypeReferenceNode('types.OperationResult', [
                    ts.factory.createTypeReferenceNode('TOperation'),
                ]),
            ],
        ),
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
                            config.sdk.buildVariablesType(
                                ts.factory.createTypeReferenceNode(
                                    'types.OperationVariables',
                                    [
                                        ts.factory.createTypeReferenceNode(
                                            'TOperation',
                                        ),
                                    ],
                                ),
                            ),
                        ),
                        ts.factory.createParameterDeclaration(
                            undefined,
                            undefined,
                            'requestContext',
                            undefined,
                            config.sdk.buildRequestContextType(
                                ts.factory.createTypeReferenceNode(
                                    'TRequestContext',
                                ),
                            ),
                        ),
                    ],
                    generateSubscriptionHookReturnType(
                        ts.factory.createTypeReferenceNode(
                            'types.OperationResult',
                            [ts.factory.createTypeReferenceNode('TOperation')],
                        ),
                    ),
                    ts.factory.createToken(
                        ts.SyntaxKind.EqualsGreaterThanToken,
                    ),
                    ts.factory.createCallExpression(
                        ts.factory.createIdentifier('useSubscription'),
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
