import { Actor, ActorContext } from '@/config.js';
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
        frameworkImportName: '@vladimirdev635/gql-client-react',
        sdk: {
            ...config.sdk,
            buildVariablesType: (variablesTypeNode) => variablesTypeNode,
            buildRequestContextType: (requestContextTypeNode) =>
                requestContextTypeNode,
        },
    });
}
