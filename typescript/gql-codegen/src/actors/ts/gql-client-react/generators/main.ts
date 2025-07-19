/* eslint-disable max-lines */
import { ActorContext } from '@/config.js';
import { GQLClientReactActorConfig } from '../actor.js';
import ts from 'typescript';

function generateFunctionBlock(
    operationName: string,
    type: 'SYNC' | 'LAZY' | 'SUBSCRIPTION'
) {
    switch (type) {
    case 'SYNC':
        return ts.factory.createCallExpression(
            ts.factory.createIdentifier('useOperation'),
            undefined,
            [
                ts.factory.createIdentifier('executor'),
                ts.factory.createIdentifier(operationName),
                ts.factory.createIdentifier('variables'),
                ts.factory.createIdentifier('requestContext')
            ]
        )
    case 'LAZY':
        return ts.factory.createCallExpression(
            ts.factory.createIdentifier('useLazyOperation'),
            undefined,
            [
                ts.factory.createIdentifier('executor'),
                ts.factory.createIdentifier(operationName)
            ]
        )
    case 'SUBSCRIPTION':
        return ts.factory.createCallExpression(
            ts.factory.createIdentifier('useSubscription'),
            undefined,
            [
                ts.factory.createIdentifier('executor'),
                ts.factory.createIdentifier(operationName),
                ts.factory.createIdentifier('variables'),
                ts.factory.createIdentifier('requestContext')
            ]
        )
    }
}

function generateArrowFunction(
    operationName: string,
    variablesName: string,
    resultName: string,
    type: 'SYNC' | 'LAZY' | 'SUBSCRIPTION'
) {
    const parameters: ts.ParameterDeclaration[] = []
    let resultType: ts.TypeNode
    switch (type) {
    case 'LAZY':
        resultType = ts.factory.createTypeReferenceNode(
            'UseLazyOperationReturnType',
            [
                ts.factory.createTypeReferenceNode(variablesName),
                ts.factory.createTypeReferenceNode(resultName),
                ts.factory.createTypeReferenceNode('TRequestContext')
            ]
        )
        break
    case 'SYNC': {
        resultType = ts.factory.createTypeReferenceNode('OperationState', [
            ts.factory.createTypeReferenceNode(resultName)
        ])
        parameters.push(
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
        )
        break
    }
    case 'SUBSCRIPTION': {
        resultType = ts.factory.createTypeReferenceNode('OperationState', [
            ts.factory.createTypeReferenceNode(
                'SubOpAsyncIterable',
                [ts.factory.createTypeReferenceNode(resultName)]
            )
        ])
        parameters.push(
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
        )
        break
    }
    }
    return ts.factory.createArrowFunction(
        undefined,
        undefined,
        parameters,
        resultType,
        ts.factory.createToken(
            ts.SyntaxKind.EqualsGreaterThanToken
        ),
        generateFunctionBlock(operationName, type)
    )
}

export function generateNodes(
    config: GQLClientReactActorConfig,
    context: ActorContext
): ts.Node[] {
    const graphqlImports: string[] = []
    const nodes = Object.values(context.schema.client.operations)
        .map(operation => {
            const operationName = operation.name + 'Operation'
            const variablesName = operation.name + 'Variables'
            const resultName = operation.name + 'Result'
            graphqlImports.push(operationName, variablesName, resultName)
            if (operation.type === 'SUBSCRIPTION') {
                return [
                    ts.factory.createPropertyAssignment(
                        'use' + operationName,
                        generateArrowFunction(
                            operationName,
                            variablesName,
                            resultName,
                            'SUBSCRIPTION'
                        )
                    ),
                ]
            }
            return [
                ts.factory.createPropertyAssignment(
                    'use' + operationName,
                    generateArrowFunction(
                        operationName,
                        variablesName,
                        resultName,
                        'SYNC'
                    )
                ),
                ts.factory.createPropertyAssignment(
                    'useLazy' + operationName,
                    generateArrowFunction(
                        operationName,
                        variablesName,
                        resultName,
                        'LAZY'
                    )
                )
            ]
        }).flat()
    return [
        ts.factory.createIdentifier('// @ts-nocheck'),
        ...config.importDeclarations,
        ts.factory.createImportDeclaration(
            [],
            ts.factory.createImportClause(
                false,
                undefined,
                ts.factory.createNamedImports([
                    ts.factory.createImportSpecifier(
                        false,
                        undefined,
                        ts.factory.createIdentifier('useOperation')
                    ),
                    ts.factory.createImportSpecifier(
                        false,
                        undefined,
                        ts.factory.createIdentifier('useLazyOperation')
                    ),
                    ts.factory.createImportSpecifier(
                        true,
                        undefined,
                        ts.factory.createIdentifier('OperationState')
                    ),
                    ts.factory.createImportSpecifier(
                        true,
                        undefined,
                        ts.factory.createIdentifier(
                            'UseLazyOperationReturnType'
                        )
                    )
                ])
            ),
            ts.factory.createStringLiteral('@vladimirdev635/gql-client-react')
        ),
        ts.factory.createImportDeclaration(
            [],
            ts.factory.createImportClause(
                true,
                undefined,
                ts.factory.createNamedImports([
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
                ])
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
                    ts.factory.createObjectLiteralExpression(nodes, true),
                )
            ], true)
        )
    ]
}
