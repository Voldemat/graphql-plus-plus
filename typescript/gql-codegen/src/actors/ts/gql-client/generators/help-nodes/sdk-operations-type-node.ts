import { Operation, OperationType } from '@/schema/client/operation.js';
import ts from 'typescript';
import { Config } from '../../actor.js';

function generateOperationPropertySignature(
    config: Config,
    type: OperationType,
    operation: Operation,
): ts.PropertySignature {
    return ts.factory.createPropertySignature(
        undefined,
        ts.factory.createIdentifier(operation.name),
        undefined,
        ts.factory.createTypeReferenceNode(
            type !== 'SUBSCRIPTION'
                ? config.sdk.gqlSyncMethodFuncTypeName
                : config.sdk.gqlSubscriptionMethodFuncTypeName,
            [
                ts.factory.createTypeReferenceNode('TRequestContext'),
                ts.factory.createTypeReferenceNode(
                    operation.name + 'Variables',
                ),
                ts.factory.createTypeReferenceNode(
                    operation.name + 'Result',
                    undefined,
                ),
            ],
        ),
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
            config.sdk.operationRequestsTypeNameBuilder(type),
        ),
        [
            ts.factory.createTypeParameterDeclaration(
                undefined,
                ts.factory.createIdentifier('TRequestContext'),
                undefined,
                undefined,
            ),
        ],
        undefined,
        Object.values(operations)
            .filter((op) => op.type == type)
            .map((op) => generateOperationPropertySignature(config, type, op)),
    );
}
