import ts from 'typescript';
import { Config } from '../../actor.js';

export function generateMethodFuncAlias(
    config: Config,
): ts.TypeAliasDeclaration {
    return ts.factory.createTypeAliasDeclaration(
        undefined,
        ts.factory.createIdentifier(config.sdk.gqlMethodFuncTypeName),
        [
            ts.factory.createTypeParameterDeclaration(
                undefined,
                ts.factory.createIdentifier('TRequestContext'),
                undefined,
                undefined,
            ),
            ts.factory.createTypeParameterDeclaration(
                undefined,
                ts.factory.createIdentifier('V'),
                undefined,
                undefined,
            ),
            ts.factory.createTypeParameterDeclaration(
                undefined,
                ts.factory.createIdentifier('R'),
                undefined,
                undefined,
            ),
        ],
        ts.factory.createFunctionTypeNode(
            undefined,
            [
                ts.factory.createParameterDeclaration(
                    undefined,
                    undefined,
                    ts.factory.createIdentifier('variables'),
                    undefined,
                    ts.factory.createTypeReferenceNode(
                        ts.factory.createIdentifier('V'),
                        undefined,
                    ),
                    undefined,
                ),
                ts.factory.createParameterDeclaration(
                    undefined,
                    undefined,
                    ts.factory.createIdentifier('context'),
                    undefined,
                    ts.factory.createTypeReferenceNode(
                        ts.factory.createIdentifier('TRequestContext'),
                        undefined,
                    ),
                    undefined,
                ),
            ],
            ts.factory.createTypeReferenceNode(
                ts.factory.createIdentifier('Promise'),
                [
                    ts.factory.createTypeReferenceNode(
                        ts.factory.createIdentifier('R'),
                        undefined,
                    ),
                ],
            ),
        ),
    );
}
