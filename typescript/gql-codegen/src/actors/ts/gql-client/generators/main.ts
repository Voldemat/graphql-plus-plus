/* eslint-disable max-lines */
import { ActorContext } from '@/config.js';
import { GQLClientActorConfig, OperationReturnType } from '../actor.js';
import ts from 'typescript';

function generateVariablesTypeNode(operationName: string) {
    return ts.factory.createTypeReferenceNode(
        'z.infer',
        [
            ts.factory.createIndexedAccessTypeNode(
                ts.factory.createParenthesizedType(
                    ts.factory.createTypeQueryNode(
                        ts.factory.createIdentifier(operationName),
                    )
                ),
                ts.factory.createLiteralTypeNode(
                    ts.factory.createStringLiteral('variablesSchema')
                )
            )
        ]
    )
}

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
                ts.factory.createIdentifier('variables')
            ]
        )
    )
    if (returnType === 'ExecutorResult') {
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

export function generateNodes(
    config: GQLClientActorConfig,
    context: ActorContext
): ts.Node[] {
    const graphqlImports: string[] = []
    const nodes = Object.values(context.schema.client.operations)
        .map(operation => {
            const operationName = operation.name + 'Operation'
            graphqlImports.push(operationName)
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
                            generateVariablesTypeNode(operationName)
                        ),

                    ],
                    undefined,
                    ts.factory.createToken(
                        ts.SyntaxKind.EqualsGreaterThanToken
                    ),
                    generateFunctionBlock(
                        operationName,
                        config.sdk.operationReturnTypeMapping[operation.name] ||
                        config.sdk.defaultOperationReturnType
                    )
                )
            )
        })
    return [
        ...config.importDeclarations,
        ts.factory.createImportDeclaration(
            [],
            ts.factory.createImportClause(
                false,
                undefined,
                ts.factory.createNamedImports([
                    ts.factory.createImportSpecifier(
                        true,
                        undefined,
                        ts.factory.createIdentifier('Executor')
                    )
                ])
            ),
            ts.factory.createStringLiteral('@vladimirdev635/gql-client')
        ),
        ts.factory.createImportDeclaration(
            [],
            ts.factory.createImportClause(
                false,
                undefined,
                ts.factory.createNamedImports([
                    ts.factory.createImportSpecifier(
                        false,
                        undefined,
                        ts.factory.createIdentifier('z')
                    )
                ])
            ),
            ts.factory.createStringLiteral('zod/v4')
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
            [],
            [ts.factory.createParameterDeclaration(
                undefined,
                undefined,
                'executor',
                undefined,
                ts.factory.createTypeReferenceNode('Executor')
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
