/* eslint-disable max-lines */
import { ActorContext } from '@/config.js';
import { ApolloActorConfig, ImportModule } from '../actor.js';
import ts from 'typescript';
import { operationSchema } from '@/schema/client/operation.js';
import { z } from 'zod/v4';
import { opTypeToName } from '../../graphql/generators/client/operations.js';
import { addNewLineBetweenNodes } from '../../shared.js';
import { ImportRegistry } from './import-registry.js';


export function generateMutationFuncTypeNode(
    registry: ImportRegistry,
    operationName: string
): ts.TypeAliasDeclaration {
    return ts.factory.createTypeAliasDeclaration(
        ts.factory.createModifiersFromModifierFlags(
            ts.ModifierFlags.Export
        ),
        operationName + 'MutationFn',
        [],
        ts.factory.createTypeReferenceNode(
            registry.addImport(ImportModule.APOLLO, 'MutationFunction'),
            [
                ts.factory.createTypeReferenceNode(
                    registry.addImport(
                        ImportModule.GRAPHQL,
                        operationName + 'Mutation'
                    )
                ),
                ts.factory.createTypeReferenceNode(
                    registry.addImport(
                        ImportModule.GRAPHQL,
                        operationName + 'MutationVariables'
                    ),
                )
            ]
        )
    )
}

function generateOperationHookFunctionNodes(
    registry: ImportRegistry,
    operation: z.infer<typeof operationSchema>,
    operationInfix: string
): ts.Node[] {
    return [
        ts.factory.createFunctionDeclaration(
            [ts.factory.createToken(ts.SyntaxKind.ExportKeyword)],
            undefined,
            ts.factory.createIdentifier(
                'use' +
                operation.name +
                operationInfix +
                opTypeToName(operation.type)
            ),
            undefined,
            [ts.factory.createParameterDeclaration(
                undefined,
                undefined,
                ts.factory.createIdentifier('baseOptions'),
                ts.factory.createToken(ts.SyntaxKind.QuestionToken),
                ts.factory.createTypeReferenceNode(
                    ts.factory.createIdentifier(
                        registry.addImport(
                            ImportModule.APOLLO,
                            opTypeToName(operation.type) + 'HookOptions'
                        )
                    ),
                    [
                        ts.factory.createTypeReferenceNode(
                            registry.addImport(
                                ImportModule.GRAPHQL,
                                operation.name + opTypeToName(operation.type)
                            ),
                        ),
                        ts.factory.createTypeReferenceNode(
                            registry.addImport(
                                ImportModule.GRAPHQL,
                                operation.name +
                                opTypeToName(operation.type) +
                                'Variables'
                            )
                        )
                    ]
                ),
                undefined
            )],
            undefined,
            ts.factory.createBlock(
                [
                    ts.factory.createVariableStatement(
                        undefined,
                        ts.factory.createVariableDeclarationList(
                            [ts.factory.createVariableDeclaration(
                                ts.factory.createIdentifier('options'),
                                undefined,
                                undefined,
                                ts.factory.createObjectLiteralExpression(
                                    [
                                        ts.factory.createSpreadAssignment(
                                            ts.factory.createIdentifier(
                                                'defaultOptions'
                                            )
                                        ),
                                        ts.factory.createSpreadAssignment(
                                            ts.factory.createIdentifier(
                                                'baseOptions'
                                            )
                                        )
                                    ],
                                    false
                                )
                            )],
                            ts.NodeFlags.Const
                        )
                    ),
                    ts.factory.createReturnStatement(
                        ts.factory.createCallExpression(
                            ts.factory.createIdentifier(
                                registry.addImport(
                                    ImportModule.APOLLO,
                                    'use' +
                                    operationInfix +
                                    opTypeToName(operation.type),
                                    { alias: null, type: false }
                                )
                            ),
                            [
                                ts.factory.createTypeReferenceNode(
                                    registry.addImport(
                                        ImportModule.GRAPHQL,
                                        operation.name +
                                        opTypeToName(operation.type)
                                    ),
                                ),
                                ts.factory.createTypeReferenceNode(
                                    registry.addImport(
                                        ImportModule.GRAPHQL,
                                        operation.name +
                                        opTypeToName(operation.type) +
                                        'Variables'
                                    ),
                                )
                            ],
                            [
                                ts.factory.createIdentifier(
                                    operation.name + 'ApolloDocument',
                                ),
                                ts.factory.createIdentifier('options')
                            ]
                        )
                    )
                ],
                true
            )
        ),
        ts.factory.createTypeAliasDeclaration(
            [ts.factory.createToken(ts.SyntaxKind.ExportKeyword)],
            ts.factory.createIdentifier(
                operation.name +
                opTypeToName(operation.type) +
                operationInfix +
                'HookResult'
            ),
            undefined,
            ts.factory.createTypeReferenceNode(
                ts.factory.createIdentifier('ReturnType'),
                [ts.factory.createTypeQueryNode(
                    ts.factory.createIdentifier(
                        'use' + operation.name + opTypeToName(operation.type)
                    ),
                    undefined
                )]
            )
        )]
}

function generateOperationDocumentNode(
    registry: ImportRegistry,
    operationName: string
) {
    return ts.factory.createVariableStatement(
        undefined,
        ts.factory.createVariableDeclarationList(
            [ts.factory.createVariableDeclaration(
                operationName + 'ApolloDocument',
                undefined,
                undefined,
                ts.factory.createCallExpression(
                    ts.factory.createIdentifier(
                        registry.addImport(
                            ImportModule.APOLLO,
                            'gql',
                            { alias: null, type: false }
                        ),
                    ),
                    undefined,
                    [ts.factory.createIdentifier(
                        registry.addImport(
                            ImportModule.GRAPHQL,
                            operationName + 'Document',
                            { alias: null, type: false }
                        ),
                    )]
                )
            )],
            ts.NodeFlags.Const
        )
    )
}


function generateQueryResultNode(
    registry: ImportRegistry,
    operation: z.infer<typeof operationSchema>
) {
    const opName = opTypeToName(operation.type)
    return ts.factory.createTypeAliasDeclaration(
        ts.factory.createModifiersFromModifierFlags(
            ts.ModifierFlags.Export
        ),
        operation.name + opName + 'Result',
        [],
        ts.factory.createTypeReferenceNode(
            registry.addImport(ImportModule.APOLLO, 'QueryResult'),
            [
                ts.factory.createTypeReferenceNode(
                    registry.addImport(
                        ImportModule.GRAPHQL,
                        operation.name + opName
                    )
                ),
                ts.factory.createTypeReferenceNode(
                    registry.addImport(
                        ImportModule.GRAPHQL,
                        operation.name + 'QueryVariables'
                    ),
                )
            ]
        )
    )
}

function generateMutationOptionsNode(
    registry: ImportRegistry,
    operation: z.infer<typeof operationSchema>
) {
    const opName = opTypeToName(operation.type)
    return ts.factory.createTypeAliasDeclaration(
        ts.factory.createModifiersFromModifierFlags(
            ts.ModifierFlags.Export
        ),
        operation.name + opName + 'Options',
        [],
        ts.factory.createTypeReferenceNode(
            registry.addImport(ImportModule.APOLLO, 'BaseMutationOptions'),
            [
                ts.factory.createTypeReferenceNode(
                    registry.addImport(
                        ImportModule.GRAPHQL,
                        operation.name + opName
                    )
                ),
                ts.factory.createTypeReferenceNode(
                    registry.addImport(
                        ImportModule.GRAPHQL,
                        operation.name + opName + 'Variables'
                    )
                )
            ]
        )
    )
}
function generateMutationResultNode(
    registry: ImportRegistry,
    operation: z.infer<typeof operationSchema>
) {
    const opName = opTypeToName(operation.type)
    return ts.factory.createTypeAliasDeclaration(
        ts.factory.createModifiersFromModifierFlags(
            ts.ModifierFlags.Export
        ),
        operation.name + opName + 'Result',
        [],
        ts.factory.createTypeReferenceNode(
            registry.addImport(ImportModule.APOLLO, 'MutationResult'),
            [
                ts.factory.createTypeReferenceNode(
                    registry.addImport(
                        ImportModule.GRAPHQL,
                        operation.name + opName
                    )
                )
            ]
        )
    )
}

function generateOperationHookNodes(
    registry: ImportRegistry,
    operation: z.infer<typeof operationSchema>
): ts.Node[] {
    const nodes = [
        generateOperationDocumentNode(registry, operation.name),
        ...generateOperationHookFunctionNodes(registry, operation, '')
    ]
    if (operation.type === 'QUERY') {
        nodes.push(
            ...generateOperationHookFunctionNodes(registry, operation, 'Lazy'),
            ...generateOperationHookFunctionNodes(
                registry,
                operation,
                'Suspense'
            ),
            generateQueryResultNode(registry, operation)
        )
    }
    if (operation.type === 'MUTATION') {
        nodes.push(
            generateMutationResultNode(registry, operation),
            generateMutationOptionsNode(registry, operation),
        )
    }
    return nodes
}

export function generateOperationNodes(
    registry: ImportRegistry,
    operation: z.infer<typeof operationSchema>
): ts.Node[] {
    const nodes: ts.Node[] = []
    if (operation.type === 'MUTATION') {
        nodes.push(generateMutationFuncTypeNode(registry, operation.name))
    }
    nodes.push(...generateOperationHookNodes(registry, operation))
    return nodes
}

export function generateNodes(
    config: ApolloActorConfig,
    context: ActorContext
): ts.Node[] {
    const registry = new ImportRegistry()
    const nodes = Object.values(context.schema.client.operations).map(op => {
        return generateOperationNodes(registry, op)
    }).flat()
    return addNewLineBetweenNodes([
        ...registry.generateImportDeclarations(config.modulePaths),
        ts.factory.createVariableStatement(
            [ts.factory.createToken(ts.SyntaxKind.ExportKeyword)],
            ts.factory.createVariableDeclarationList(
                [ts.factory.createVariableDeclaration(
                    ts.factory.createIdentifier('defaultOptions'),
                    undefined,
                    undefined,
                    ts.factory.createObjectLiteralExpression()
                )],
                ts.NodeFlags.Const
            )
        ),
        ...nodes
    ])
}
