import { Operation } from '@/schema/client/operation.js';
import ts from 'typescript';
import { Config } from '../../actor.js';

function generateOperationPropertySignature(
    config: Config,
    type: Operation['type'],
    operation: Operation,
): ts.PropertySignature {
    return ts.factory.createPropertySignature(
        undefined,
        ts.factory.createIdentifier(operation.name),
        undefined,
        ts.factory.createTypeReferenceNode(config.sdk.gqlMethodFuncTypeName, [
            ts.factory.createTypeReferenceNode('TRequestContext'),
            ts.factory.createTypeReferenceNode(operation.name + 'Variables'),
            type != 'SUBSCRIPTION'
                ? ts.factory.createTypeReferenceNode(
                      operation.name + 'Result',
                      undefined,
                  )
                : ts.factory.createTypeReferenceNode('SubOpAsyncIterable', [
                      ts.factory.createTypeReferenceNode(
                          operation.name + 'Result',
                      ),
                  ]),
        ]),
    );
}

export function generateSdkTypeNode(
    config: Config,
    type: Operation['type'],
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
