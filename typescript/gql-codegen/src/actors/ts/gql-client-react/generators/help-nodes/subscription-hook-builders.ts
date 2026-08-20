/* oxlint-disable max-lines */
import ts from 'typescript';
import { Config } from '../../actor.js';
import { generateHookReturnType } from '../hook-return-type.js';

function generateSubscriptionHookReturnType(config: Config): ts.TypeNode {
    return generateHookReturnType(
        config,
        'SUBSCRIPTION',
        'TVariables',
        'TRequestContext',
    );
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
            generateSubscriptionHookReturnType(config),
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
                ts.factory.createTypeReferenceNode('SubscriptionOperation', [
                    ts.factory.createTypeReferenceNode('TVariables'),
                    ts.factory.createTypeReferenceNode('TResult'),
                ]),
            ),
        ],
        ts.factory.createTypeReferenceNode(
            config.sdk.subscriptionHookTypeName,
            [
                ts.factory.createTypeReferenceNode('TRequestContext'),
                ts.factory.createTypeReferenceNode('TVariables'),
                ts.factory.createTypeReferenceNode('TResult'),
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
                    generateSubscriptionHookReturnType(config),
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
