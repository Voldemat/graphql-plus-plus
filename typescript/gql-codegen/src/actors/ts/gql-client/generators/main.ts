/* eslint-disable max-lines */
import { ActorContext } from '@/config.js';
import { GQLClientActorConfig, OperationReturnType } from '../actor.js';
import ts from 'typescript';
import { operationSchema } from '@/schema/client/operation.js';
import { z } from 'zod/v4';

function generateFunctionBlock(
    operationName: string,
    operationType: z.infer<typeof operationSchema>['type'],
    returnType: OperationReturnType
) {
    const callArgs = [
        ts.factory.createIdentifier(operationName),
        ts.factory.createIdentifier('variables'),
        ts.factory.createIdentifier('requestContext')
    ]
    if (operationType === 'SUBSCRIPTION') {
        callArgs.push(ts.factory.createIdentifier('controller'))
    }
    const awaitExpression = ts.factory.createAwaitExpression(
        ts.factory.createCallExpression(
            ts.factory.createPropertyAccessExpression(
                ts.factory.createIdentifier('executor'),
                operationType === 'SUBSCRIPTION' ?
                    'executeSubscription' :
                    'executeSync'
            ),
            undefined,
            callArgs
        )
    )
    if (returnType === 'ExecuteResult') {
        return ts.factory.createBlock([
            ts.factory.createReturnStatement(awaitExpression)
        ], true)
    }
    return ts.factory.createBlock([
        ts.factory.createVariableStatement(
            undefined,
            ts.factory.createVariableDeclarationList([
                ts.factory.createVariableDeclaration(
                    'executorResult',
                    undefined,
                    undefined,
                    awaitExpression
                ),
            ], ts.NodeFlags.Const)
        ),
        ts.factory.createReturnStatement(
            ts.factory.createPropertyAccessChain(
                ts.factory.createIdentifier('executorResult'),
                undefined,
                'result'
            )
        )
    ], true)
}

function createReturnTypeNode(
    resultName: string,
    returnType: OperationReturnType,
    operationType: 'QUERY' | 'MUTATION' | 'SUBSCRIPTION'
) {
    const rOpType = ts.factory.createTypeReferenceNode(resultName)
    const rType = operationType === 'SUBSCRIPTION' ?
        ts.factory.createTypeReferenceNode('SubOpAsyncIterable', [rOpType]) :
        rOpType
    if (returnType === 'ExecuteResult.result') return rType
    return ts.factory.createTypeReferenceNode('ExecuteResult', [rType])
}

function getReturnTypeFromConfig(
    config: GQLClientActorConfig,
    operationName: string
) {
    return config.sdk.operationReturnTypeMapping[operationName] ||
        config.sdk.defaultOperationReturnType
}

export function generateNodes(
    config: GQLClientActorConfig,
    context: ActorContext
): ts.Node[] {
    const graphqlImports: string[] = []
    let shouldIncludeExecuteResultType = false
    const queryNodes: ts.PropertyAssignment[] = []
    const mutationNodes: ts.PropertyAssignment[] = []
    const subscriptionNodes: ts.PropertyAssignment[] = []
    for (const operation of Object.values(context.schema.client.operations)) {
        const operationName = operation.name + 'Operation'
        const variablesName = operation.name + 'Variables'
        const resultName = operation.name + 'Result'
        graphqlImports.push(operationName, variablesName, resultName)
        const returnType = getReturnTypeFromConfig(config, operation.name)
        if (returnType === 'ExecuteResult')
            shouldIncludeExecuteResultType = true
        const funcArgs = [
            ts.factory.createParameterDeclaration(
                undefined,
                undefined,
                'variables',
                undefined,
                ts.factory.createTypeReferenceNode(variablesName)
            ),
            ts.factory.createParameterDeclaration(
                undefined,
                undefined,
                'requestContext',
                undefined,
                ts.factory.createTypeReferenceNode(
                    'TRequestContext'
                ),
            ),
        ]
        if (operation.type === 'SUBSCRIPTION') {
            funcArgs.push(
                ts.factory.createParameterDeclaration(
                    undefined,
                    undefined,
                    'controller',
                    undefined,
                    ts.factory.createTypeReferenceNode('AbortController'),
                ),
            )
        }
        const propAssignment = ts.factory.createPropertyAssignment(
            operation.name,
            ts.factory.createArrowFunction(
                ts.factory.createModifiersFromModifierFlags(
                    ts.ModifierFlags.Async
                ),
                undefined,
                funcArgs,
                ts.factory.createTypeReferenceNode(
                    'Promise',
                    [
                        createReturnTypeNode(
                            resultName,
                            returnType,
                            operation.type
                        )
                    ]
                ),
                ts.factory.createToken(
                    ts.SyntaxKind.EqualsGreaterThanToken
                ),
                generateFunctionBlock(operationName, operation.type, returnType)
            )
        )

        switch (operation.type) {
        case 'QUERY': {
            queryNodes.push(propAssignment)
            break
        }
        case 'MUTATION': {
            mutationNodes.push(propAssignment)
            break
        }
        case 'SUBSCRIPTION': {
            subscriptionNodes.push(propAssignment)
            break
        }
        }
    }

    const gqlClientImports = [
        ts.factory.createImportSpecifier(
            false,
            undefined,
            ts.factory.createIdentifier('IExecutor')
        ),
        ts.factory.createImportSpecifier(
            false,
            undefined,
            ts.factory.createIdentifier('RequestContext')
        )
    ]

    if (shouldIncludeExecuteResultType) {
        gqlClientImports.push(
            ts.factory.createImportSpecifier(
                false,
                undefined,
                ts.factory.createIdentifier('ExecuteResult')
            ),
        )
    }

    const returnObjectNodes: ts.PropertyAssignment[] = []
    if (queryNodes.length !== 0) {
        returnObjectNodes.push(
            ts.factory.createPropertyAssignment(
                config.sdk.queriesKey,
                ts.factory.createObjectLiteralExpression(queryNodes, true)
            )
        )
    }
    if (mutationNodes.length !== 0) {
        returnObjectNodes.push(
            ts.factory.createPropertyAssignment(
                config.sdk.mutationsKey,
                ts.factory.createObjectLiteralExpression(mutationNodes, true)
            )
        )
    }
    if (subscriptionNodes.length !== 0) {
        gqlClientImports.push(
            ts.factory.createImportSpecifier(
                false,
                undefined,
                ts.factory.createIdentifier('SubOpAsyncIterable')
            ),
        )
        returnObjectNodes.push(
            ts.factory.createPropertyAssignment(
                config.sdk.subscriptionsKey,
                ts.factory.createObjectLiteralExpression(
                    subscriptionNodes, true
                )
            )
        )
    }

    return [
        ts.factory.createIdentifier('// @ts-nocheck'),
        ...config.importDeclarations,
        ts.factory.createImportDeclaration(
            [],
            ts.factory.createImportClause(
                true,
                undefined,
                ts.factory.createNamedImports(gqlClientImports)
            ),
            ts.factory.createStringLiteral('@vladimirdev635/gql-client/types')
        ),
        ts.factory.createImportDeclaration(
            undefined,
            ts.factory.createImportClause(
                false,
                undefined,
                ts.factory.createNamedImports(graphqlImports.map(i =>
                    ts.factory.createImportSpecifier(
                        false,
                        undefined,
                        ts.factory.createIdentifier(i)
                    )))
            ),
            ts.factory.createStringLiteral(config.graphqlModulePath)
        ),
        ts.factory.createIdentifier('\n'),
        ts.factory.createFunctionDeclaration(
            ts.factory.createModifiersFromModifierFlags(
                ts.ModifierFlags.Export
            ),
            undefined,
            'createSdk',
            [ts.factory.createTypeParameterDeclaration(
                undefined,
                'TRequestContext',
                ts.factory.createTypeReferenceNode('RequestContext')
            )],
            [ts.factory.createParameterDeclaration(
                undefined,
                undefined,
                'executor',
                undefined,
                ts.factory.createTypeReferenceNode('IExecutor', [
                    ts.factory.createTypeReferenceNode('TRequestContext')
                ])
            )],
            undefined,
            ts.factory.createBlock([
                ts.factory.createReturnStatement(
                    ts.factory.createAsExpression(
                        ts.factory.createObjectLiteralExpression(
                            returnObjectNodes, true
                        ),
                        ts.factory.createTypeReferenceNode('const')
                    )
                )
            ], true)
        )
    ]
}
