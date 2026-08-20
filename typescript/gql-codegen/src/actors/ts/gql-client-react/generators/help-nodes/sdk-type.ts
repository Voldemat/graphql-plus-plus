import ts from 'typescript';
import { Config } from '../../actor.js';

function generateSdkPropertySignature(
    key: string,
    typeName: string,
): ts.PropertySignature {
    return ts.factory.createPropertySignature(
        undefined,
        ts.factory.createIdentifier(key),
        undefined,
        ts.factory.createTypeReferenceNode(
            ts.factory.createIdentifier(typeName),
            [
                ts.factory.createTypeReferenceNode(
                    ts.factory.createIdentifier('TRequestContext'),
                ),
            ],
        ),
    );
}

export function generateSdkType(
    config: Config,
    state: {
        hasQueries: boolean;
        hasMutations: boolean;
        hasSubscriptions: boolean;
    },
): ts.InterfaceDeclaration {
    const properties = [];
    if (state.hasQueries) {
        properties.push(
            generateSdkPropertySignature(
                config.sdk.queriesKey,
                config.sdk.operationHooksTypeNameBuilder('QUERY'),
            ),
        );
    }
    if (state.hasMutations) {
        properties.push(
            generateSdkPropertySignature(
                config.sdk.mutationsKey,
                config.sdk.operationHooksTypeNameBuilder('MUTATION'),
            ),
        );
    }
    if (state.hasSubscriptions) {
        properties.push(
            generateSdkPropertySignature(
                config.sdk.subscriptionsKey,
                config.sdk.operationHooksTypeNameBuilder('SUBSCRIPTION'),
            ),
        );
    }
    return ts.factory.createInterfaceDeclaration(
        [ts.factory.createToken(ts.SyntaxKind.ExportKeyword)],
        ts.factory.createIdentifier(config.sdk.typeName),
        [
            ts.factory.createTypeParameterDeclaration(
                undefined,
                ts.factory.createIdentifier('TRequestContext'),
                ts.factory.createTypeReferenceNode('RequestContext'),
                undefined,
            ),
        ],
        undefined,
        properties,
    );
}
