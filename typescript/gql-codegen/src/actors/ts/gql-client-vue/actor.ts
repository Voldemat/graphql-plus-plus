import { Actor, ActorContext } from '@/config.js';
import ts from 'typescript';
import {
    Config as FrameworkConfig,
    SDKConfig as FrameworkSDKConfig,
    build as frameworkBuild,
} from '../gql-client-framework/index.js';
import { TSActorConfig } from '../shared.js';

export interface SDKConfig extends Pick<
    FrameworkSDKConfig,
    | 'typeName'
    | 'queriesKey'
    | 'mutationsKey'
    | 'subscriptionsKey'
    | 'clientTypeNameBuilders'
    | 'hookNameBuilders'
    | 'operationHooksTypeNameBuilder'
    | 'lazyHookBuilderName'
    | 'lazyHookTypeName'
    | 'syncHookBuilderName'
    | 'syncHookTypeName'
    | 'subscriptionHookBuilderName'
    | 'subscriptionHookTypeName'
> {}

export interface Config extends Pick<
    FrameworkConfig,
    'outPath' | 'importDeclarations' | 'graphqlModulePath' | keyof TSActorConfig
> {
    sdk: SDKConfig;
}

export function build(config: Config): Actor<ActorContext> {
    return frameworkBuild({
        ...config,
        importDeclarations: [
            ...config.importDeclarations,
            ts.factory.createImportDeclaration(
                undefined,
                ts.factory.createImportClause(
                    ts.SyntaxKind.TypeKeyword,
                    undefined,
                    ts.factory.createNamedImports([
                        ts.factory.createImportSpecifier(
                            false,
                            undefined,
                            ts.factory.createIdentifier('Ref'),
                        ),
                    ]),
                ),
                ts.factory.createStringLiteral('vue'),
            ),
        ],
        frameworkImportName: '@vladimirdev635/gql-client-vue',
        sdk: {
            ...config.sdk,
            buildVariablesType: (variablesTypeNode) =>
                ts.factory.createUnionTypeNode([
                    variablesTypeNode,
                    ts.factory.createTypeReferenceNode('Ref', [
                        variablesTypeNode,
                    ]),
                ]),
            buildRequestContextType: (requestContextTypeNode) =>
                ts.factory.createUnionTypeNode([
                    requestContextTypeNode,
                    ts.factory.createTypeReferenceNode('Ref', [
                        requestContextTypeNode,
                    ]),
                ]),
        },
    });
}
