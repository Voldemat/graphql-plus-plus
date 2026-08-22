/* eslint-disable max-lines */
import { ActorContext } from '@/config.js';
import ts from 'typescript';
import { Config } from '../actor.js';
import { createBuildSubscriptionCallbackFunctionName } from './help-nodes/build-subscription-callback-function.js';
import { createBuildSyncCallbackFunctionName } from './help-nodes/build-sync-callback-function.js';
import { generateHelpNodes } from './help-nodes/index.js';
import { getReturnTypeFromConfig } from './operation-return-type.js';

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
        const operationReturnType = getReturnTypeFromConfig(
            config,
            operation.name,
        );
        const functionName =
            operation.type === 'SUBSCRIPTION'
                ? createBuildSubscriptionCallbackFunctionName(
                      operationReturnType,
                  )
                : createBuildSyncCallbackFunctionName(operationReturnType);
        const propAssignment = ts.factory.createPropertyAssignment(
            operation.name,
            ts.factory.createCallExpression(
                ts.factory.createIdentifier(functionName),
                [
                    ts.factory.createTypeReferenceNode('TExecutor', undefined),
                    ts.factory.createTypeReferenceNode(
                        'TRequestContext',
                        undefined,
                    ),
                    ts.factory.createTypeReferenceNode(
                        variablesName,
                        undefined,
                    ),
                    ts.factory.createTypeReferenceNode(resultName, undefined),
                ],
                [
                    ts.factory.createIdentifier('executor'),
                    ts.factory.createIdentifier(operationName),
                ],
            ),
        );

        switch (operation.type) {
            case 'QUERY': {
                queryNodes.push(propAssignment);
                break;
            }
            case 'MUTATION': {
                mutationNodes.push(propAssignment);
                break;
            }
            case 'SUBSCRIPTION': {
                subscriptionNodes.push(propAssignment);
                break;
            }
        }
    }

    const gqlClientImports = [
        ts.factory.createImportSpecifier(
            false,
            undefined,
            ts.factory.createIdentifier('types'),
        ),
    ];

    const returnObjectNodes: ts.PropertyAssignment[] = [];
    const state = {
        hasQueries: queryNodes.length !== 0,
        hasMutations: mutationNodes.length !== 0,
        hasSubscriptions: subscriptionNodes.length !== 0,
    };
    if (state.hasQueries) {
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
    if (state.hasSubscriptions) {
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
                true,
                undefined,
                ts.factory.createNamedImports(gqlClientImports),
            ),
            ts.factory.createStringLiteral('@vladimirdev635/gql-client'),
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
            ts.factory.createTypeReferenceNode('SdkType', [
                ts.factory.createTypeReferenceNode('TRequestContext'),
            ]),
            ts.factory.createBlock(
                [
                    ts.factory.createReturnStatement(
                        ts.factory.createAsExpression(
                            ts.factory.createObjectLiteralExpression(
                                returnObjectNodes,
                                true,
                            ),
                            ts.factory.createTypeReferenceNode('const'),
                        ),
                    ),
                ],
                true,
            ),
        ),
    ];
}
