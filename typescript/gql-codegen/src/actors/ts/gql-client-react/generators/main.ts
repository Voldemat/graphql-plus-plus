/* eslint-disable max-lines */
import { ActorContext } from '@/config.js';
import { GQLClientReactActorConfig } from '../actor.js';
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
    lazy: boolean
) {
    if (!lazy) {
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
    }
    return ts.factory.createCallExpression(
        ts.factory.createIdentifier('useLazyOperation'),
        undefined,
        [
            ts.factory.createIdentifier('executor'),
            ts.factory.createIdentifier(operationName)
        ]
    )
}

function generateArrowFunction(
    operationName: string,
    lazy: boolean
) {
    const parameters: ts.ParameterDeclaration[] = []
    if (!lazy) {
        parameters.push(
            ts.factory.createParameterDeclaration(
                undefined,
                undefined,
                'variables',
                undefined,
                generateVariablesTypeNode(operationName)
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
    }
    return ts.factory.createArrowFunction(
        undefined,
        undefined,
        parameters,
        undefined,
        ts.factory.createToken(
            ts.SyntaxKind.EqualsGreaterThanToken
        ),
        generateFunctionBlock(operationName, lazy)
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
            graphqlImports.push(operationName)
            return [
                ts.factory.createPropertyAssignment(
                    'use' + operationName,
                    generateArrowFunction(operationName, false)
                ),
                ts.factory.createPropertyAssignment(
                    'useLazy' + operationName,
                    generateArrowFunction(operationName, true)
                )
            ]
        }).flat()
    return [
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
