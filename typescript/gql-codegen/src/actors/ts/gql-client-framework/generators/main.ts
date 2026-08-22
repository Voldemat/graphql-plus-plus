/* eslint-disable max-lines */
import { ActorContext } from '@/config.js';
import ts from 'typescript';
import { Config } from '../actor.js';
import { generateHelpNodes } from './help-nodes/index.js';

function getHookBuilderName(
    config: Config,
    type: 'SYNC' | 'LAZY' | 'SUBSCRIPTION',
): string {
    switch (type) {
        case 'SYNC':
            return config.sdk.syncHookBuilderName;
        case 'LAZY':
            return config.sdk.lazyHookBuilderName;
        case 'SUBSCRIPTION':
            return config.sdk.subscriptionHookBuilderName;
    }
}

function generateHookValueExpression(
    config: Config,
    operationName: string,
    type: 'SYNC' | 'LAZY' | 'SUBSCRIPTION',
) {
    return ts.factory.createCallExpression(
        ts.factory.createIdentifier(getHookBuilderName(config, type)),
        undefined,
        [
            ts.factory.createIdentifier('executor'),
            ts.factory.createIdentifier(operationName),
        ],
    );
}

export function generateNodes(
    config: Config,
    context: ActorContext,
): ts.Node[] {
    const graphqlImports: ts.ImportSpecifier[] = [];
    const queryNodes: ts.PropertyAssignment[] = [];
    const mutationNodes: ts.PropertyAssignment[] = [];
    const subscriptionNodes: ts.PropertyAssignment[] = [];
    for (const operation of Object.values(context.schema.client.operations)) {
        const operationName =
            config.sdk.clientTypeNameBuilders.operationTypeName(operation.name);
        const variablesName =
            config.sdk.clientTypeNameBuilders.variablesTypeName(operation.name);
        const resultName = config.sdk.clientTypeNameBuilders.resultTypeName(
            operation.name,
        );
        graphqlImports.push(
            ts.factory.createImportSpecifier(
                false,
                undefined,
                ts.factory.createIdentifier(operationName),
            ),
            ts.factory.createImportSpecifier(
                true,
                undefined,
                ts.factory.createIdentifier(variablesName),
            ),
            ts.factory.createImportSpecifier(
                true,
                undefined,
                ts.factory.createIdentifier(resultName),
            ),
        );
        switch (operation.type) {
            case 'SUBSCRIPTION': {
                subscriptionNodes.push(
                    ts.factory.createPropertyAssignment(
                        config.sdk.hookNameBuilders.subscription.immediate(
                            operation.name,
                        ),
                        generateHookValueExpression(
                            config,
                            operationName,
                            'SUBSCRIPTION',
                        ),
                    ),
                );
                break;
            }
            case 'MUTATION': {
                mutationNodes.push(
                    ts.factory.createPropertyAssignment(
                        config.sdk.hookNameBuilders.mutation.lazy(
                            operation.name,
                        ),
                        generateHookValueExpression(
                            config,
                            operationName,
                            'LAZY',
                        ),
                    ),
                );
                break;
            }
            case 'QUERY': {
                queryNodes.push(
                    ts.factory.createPropertyAssignment(
                        config.sdk.hookNameBuilders.query.immediate(
                            operation.name,
                        ),
                        generateHookValueExpression(
                            config,
                            operationName,
                            'SYNC',
                        ),
                    ),
                    ts.factory.createPropertyAssignment(
                        config.sdk.hookNameBuilders.query.lazy(operation.name),
                        generateHookValueExpression(
                            config,
                            operationName,
                            'LAZY',
                        ),
                    ),
                );
                break;
            }
        }
    }
    const gqlClientReactImports: ts.ImportSpecifier[] = [];

    const state = {
        hasQueries: queryNodes.length !== 0,
        hasMutations: mutationNodes.length !== 0,
        hasSubscriptions: subscriptionNodes.length !== 0,
    };

    const returnObjectNodes: ts.PropertyAssignment[] = [];
    if (state.hasQueries) {
        gqlClientReactImports.push(
            ts.factory.createImportSpecifier(
                false,
                undefined,
                ts.factory.createIdentifier('useOperation'),
            ),
        );
        gqlClientReactImports.push(
            ts.factory.createImportSpecifier(
                true,
                undefined,
                ts.factory.createIdentifier('UseOperationHookReturnType'),
            ),
        );
        gqlClientReactImports.push(
            ts.factory.createImportSpecifier(
                true,
                undefined,
                ts.factory.createIdentifier('types'),
            ),
        );
        returnObjectNodes.push(
            ts.factory.createPropertyAssignment(
                config.sdk.queriesKey,
                ts.factory.createObjectLiteralExpression(queryNodes, true),
            ),
        );
    }
    if (state.hasMutations) {
        returnObjectNodes.push(
            ts.factory.createPropertyAssignment(
                config.sdk.mutationsKey,
                ts.factory.createObjectLiteralExpression(mutationNodes, true),
            ),
        );
    }

    if (state.hasQueries || state.hasMutations) {
        gqlClientReactImports.push(
            ts.factory.createImportSpecifier(
                false,
                undefined,
                ts.factory.createIdentifier('useLazyOperation'),
            ),
        );
        gqlClientReactImports.push(
            ts.factory.createImportSpecifier(
                true,
                undefined,
                ts.factory.createIdentifier('UseLazyOperationHookReturnType'),
            ),
        );
    }
    if (state.hasSubscriptions) {
        gqlClientReactImports.push(
            ts.factory.createImportSpecifier(
                false,
                undefined,
                ts.factory.createIdentifier('useSubscription'),
            ),
            ts.factory.createImportSpecifier(
                true,
                undefined,
                ts.factory.createIdentifier('UseSubscriptionHookReturnType'),
            ),
        );
        returnObjectNodes.push(
            ts.factory.createPropertyAssignment(
                config.sdk.subscriptionsKey,
                ts.factory.createObjectLiteralExpression(
                    subscriptionNodes,
                    true,
                ),
            ),
        );
    }

    return [
        ...config.importDeclarations,
        ts.factory.createImportDeclaration(
            [],
            ts.factory.createImportClause(
                undefined,
                undefined,
                ts.factory.createNamedImports(gqlClientReactImports),
            ),
            ts.factory.createStringLiteral(config.frameworkImportName),
        ),
        ts.factory.createImportDeclaration(
            undefined,
            ts.factory.createImportClause(
                undefined,
                undefined,
                ts.factory.createNamedImports(graphqlImports),
            ),
            ts.factory.createStringLiteral(config.graphqlModulePath),
        ),
        ts.factory.createIdentifier('\n'),
        ...generateHelpNodes(config, context, state),
        ts.factory.createIdentifier('\n'),
        ts.factory.createFunctionDeclaration(
            ts.factory.createModifiersFromModifierFlags(
                ts.ModifierFlags.Export,
            ),
            undefined,
            'createSdk',
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
            ],
            [
                ts.factory.createParameterDeclaration(
                    undefined,
                    undefined,
                    'executor',
                    undefined,
                    ts.factory.createTypeReferenceNode('TExecutor'),
                ),
            ],
            ts.factory.createTypeReferenceNode(config.sdk.typeName, [
                ts.factory.createTypeReferenceNode('TRequestContext'),
            ]),
            ts.factory.createBlock(
                [
                    ts.factory.createReturnStatement(
                        ts.factory.createObjectLiteralExpression(
                            returnObjectNodes,
                            true,
                        ),
                    ),
                ],
                true,
            ),
        ),
    ];
}
