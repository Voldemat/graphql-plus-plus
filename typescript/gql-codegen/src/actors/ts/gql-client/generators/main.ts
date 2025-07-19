/* eslint-disable max-lines */
import { ActorContext } from '@/config.js';
import { GQLClientActorConfig, OperationReturnType } from '../actor.js';
import ts from 'typescript';

function generateFunctionBlock(
    operationName: string,
    returnType: OperationReturnType
) {
    const awaitExpression = ts.factory.createAwaitExpression(
        ts.factory.createCallExpression(
            ts.factory.createIdentifier('executor'),
            undefined,
            [
                ts.factory.createIdentifier(operationName),
                ts.factory.createIdentifier('variables'),
                ts.factory.createIdentifier('requestContext')
            ]
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
    let shouldIncludeSubOpAsyncIterable = false
    const nodes = Object.values(context.schema.client.operations)
        .map(operation => {
            if (operation.type === 'SUBSCRIPTION') {
                shouldIncludeSubOpAsyncIterable = true
            }
            const operationName = operation.name + 'Operation'
            const variablesName = operation.name + 'Variables'
            const resultName = operation.name + 'Result'
            graphqlImports.push(operationName, variablesName, resultName)
            const returnType = getReturnTypeFromConfig(config, operation.name)
            if (returnType === 'ExecuteResult')
                shouldIncludeExecuteResultType = true
            return ts.factory.createPropertyAssignment(
                operation.name,
                ts.factory.createArrowFunction(
                    ts.factory.createModifiersFromModifierFlags(
                        ts.ModifierFlags.Async
                    ),
                    undefined,
                    [
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
                    ],
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
                    generateFunctionBlock(operationName, returnType)
                )
            )
        })
    const gqlClientImports = [
        ts.factory.createImportSpecifier(
            false,
            undefined,
            ts.factory.createIdentifier('Executor')
        ),
        ts.factory.createImportSpecifier(
            false,
            undefined,
            ts.factory.createIdentifier('RequestContext')
        )
    ]
    if (shouldIncludeSubOpAsyncIterable) {
        gqlClientImports.push(
            ts.factory.createImportSpecifier(
                true,
                undefined,
                ts.factory.createIdentifier('SubOpAsyncIterable')
            ),
        )
    }
    if (shouldIncludeExecuteResultType) {
        gqlClientImports.push(
            ts.factory.createImportSpecifier(
                true,
                undefined,
                ts.factory.createIdentifier('ExecuteResult')
            ),
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
            ts.factory.createStringLiteral('@vladimirdev635/gql-client')
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
                ts.factory.createTypeReferenceNode('Executor', [
                    ts.factory.createTypeReferenceNode('TRequestContext')
                ])
            )],
            undefined,
            ts.factory.createBlock([
                ts.factory.createReturnStatement(
                    ts.factory.createAsExpression(
                        ts.factory.createObjectLiteralExpression(nodes, true),
                        ts.factory.createTypeReferenceNode('const')
                    )
                )
            ], true)
        )
    ]
}
