/* oxlint-disable max-lines */
import { Operation, OperationType } from '@/schema/client/operation.js';
import ts from 'typescript';
import { assertUnreachable } from '../../../../../utils.js';
import { Config } from '../../actor.js';

function getHookTypeName(
    config: Config,
    type: 'SYNC' | 'LAZY' | 'SUBSCRIPTION',
): string {
    switch (type) {
        case 'SYNC':
            return config.sdk.syncHookTypeName;
        case 'LAZY':
            return config.sdk.lazyHookTypeName;
        case 'SUBSCRIPTION':
            return config.sdk.subscriptionHookTypeName;
    }
}

function generateOperationPropertySignature(
    config: Config,
    hookName: string,
    type: 'SYNC' | 'LAZY' | 'SUBSCRIPTION',
    operation: Operation,
): ts.PropertySignature {
    return ts.factory.createPropertySignature(
        undefined,
        ts.factory.createIdentifier(hookName),
        undefined,
        ts.factory.createTypeReferenceNode(getHookTypeName(config, type), [
            ts.factory.createTypeReferenceNode('TRequestContext'),
            ts.factory.createTypeReferenceNode(
                config.sdk.clientTypeNameBuilders.variablesTypeName(
                    operation.name,
                ),
            ),
            ts.factory.createTypeReferenceNode(
                config.sdk.clientTypeNameBuilders.resultTypeName(
                    operation.name,
                ),
            ),
        ]),
    );
}

export function generateSdkTypeNode(
    config: Config,
    type: OperationType,
    operations: Record<string, Operation>,
): ts.InterfaceDeclaration {
    return ts.factory.createInterfaceDeclaration(
        [ts.factory.createToken(ts.SyntaxKind.ExportKeyword)],
        ts.factory.createIdentifier(
            config.sdk.operationHooksTypeNameBuilder(type),
        ),
        [
            ts.factory.createTypeParameterDeclaration(
                undefined,
                ts.factory.createIdentifier('TRequestContext'),
                ts.factory.createTypeReferenceNode('RequestContext'),
                undefined,
            ),
        ],
        undefined,
        Object.values(operations)
            .filter((op) => op.type == type)
            .map((op) => {
                switch (op.type) {
                    case 'QUERY':
                        return [
                            generateOperationPropertySignature(
                                config,
                                config.sdk.hookNameBuilders.query.immediate(
                                    op.name,
                                ),
                                'SYNC',
                                op,
                            ),
                            generateOperationPropertySignature(
                                config,
                                config.sdk.hookNameBuilders.query.lazy(op.name),
                                'LAZY',
                                op,
                            ),
                        ];
                    case 'MUTATION':
                        return [
                            generateOperationPropertySignature(
                                config,
                                config.sdk.hookNameBuilders.mutation.lazy(
                                    op.name,
                                ),
                                'LAZY',
                                op,
                            ),
                        ];
                    case 'SUBSCRIPTION':
                        return [
                            generateOperationPropertySignature(
                                config,
                                config.sdk.hookNameBuilders.subscription.immediate(
                                    op.name,
                                ),
                                'SUBSCRIPTION',
                                op,
                            ),
                        ];
                    default:
                        return assertUnreachable(op.type);
                }
            })
            .flat(),
    );
}
