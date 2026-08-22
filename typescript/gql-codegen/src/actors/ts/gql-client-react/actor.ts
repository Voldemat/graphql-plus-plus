import { Actor, ActorContext } from '@/config.js';
import {
    Config as FrameworkConfig,
    SDKConfig as FrameworkSDKConfig,
    build as frameworkBuild,
} from '../gql-client-framework/index.js';

export interface SDKConfig extends Omit<
    FrameworkSDKConfig,
    'buildVariablesType'
> {}

export interface Config extends Omit<
    FrameworkConfig,
    'frameworkImportName' | 'sdk'
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
